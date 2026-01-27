# CIRISRegistry Staging Environment
# Terraform configuration for staging deployment

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
  }

  backend "s3" {
    bucket         = "ciris-terraform-state"
    key            = "registry/staging/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "ciris-terraform-locks"
  }
}

provider "aws" {
  region = var.region

  default_tags {
    tags = {
      Project     = "CIRISRegistry"
      Environment = "staging"
      ManagedBy   = "Terraform"
      Team        = "CIRISBridge"
    }
  }
}

locals {
  name_prefix = "ciris-registry-staging"
  environment = "staging"

  availability_zones = [
    "${var.region}a",
    "${var.region}b"
  ]

  tags = {
    Environment = local.environment
    Service     = "CIRISRegistry"
  }
}

# VPC Module
module "vpc" {
  source = "../../modules/vpc"

  name_prefix          = local.name_prefix
  region               = var.region
  vpc_cidr             = "10.10.0.0/16"
  availability_zones   = local.availability_zones
  enable_nat_gateway   = true
  enable_vpc_endpoints = true
  tags                 = local.tags
}

# Secrets Module
module "secrets" {
  source = "../../modules/secrets"

  name_prefix            = local.name_prefix
  environment            = local.environment
  db_host                = module.rds.db_host
  db_port                = 5432
  db_username            = "ciris_admin"
  db_name                = "ciris_registry"
  use_generated_password = true
  use_generated_jwt      = true
  enable_mtls            = false
  tags                   = local.tags
}

# RDS Module
module "rds" {
  source = "../../modules/rds"

  name_prefix         = local.name_prefix
  environment         = local.environment
  database_subnet_ids = module.vpc.database_subnet_ids
  security_group_id   = module.vpc.rds_security_group_id
  kms_key_arn         = module.secrets.kms_key_arn

  instance_class        = "db.t3.medium"
  allocated_storage     = 20
  max_allocated_storage = 100
  postgres_version      = "15.4"

  db_password = module.secrets.generated_db_password
  multi_az    = false

  enable_performance_insights = true
  enable_enhanced_monitoring  = true
  backup_retention_days       = 7

  tags = local.tags
}

# ACM Certificate
resource "aws_acm_certificate" "main" {
  domain_name               = "staging.registry.${var.domain_name}"
  subject_alternative_names = ["api.staging.registry.${var.domain_name}"]
  validation_method         = "DNS"

  lifecycle {
    create_before_destroy = true
  }

  tags = local.tags
}

# ECR Repository
resource "aws_ecr_repository" "registry" {
  name                 = "ciris-registry-staging"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  encryption_configuration {
    encryption_type = "KMS"
    kms_key         = module.secrets.kms_key_arn
  }

  tags = local.tags
}

# ECS Module
module "ecs" {
  source = "../../modules/ecs"

  name_prefix           = local.name_prefix
  environment           = local.environment
  region                = var.region
  vpc_id                = module.vpc.vpc_id
  public_subnet_ids     = module.vpc.public_subnet_ids
  private_subnet_ids    = module.vpc.private_subnet_ids
  alb_security_group_id = module.vpc.alb_security_group_id
  ecs_security_group_id = module.vpc.ecs_security_group_id

  ecr_repository_url = aws_ecr_repository.registry.repository_url
  image_tag          = var.image_tag
  certificate_arn    = aws_acm_certificate.main.arn

  task_cpu      = 512
  task_memory   = 1024
  desired_count = 1
  min_capacity  = 1
  max_capacity  = 4

  db_secret_arn  = module.secrets.db_secret_arn
  jwt_secret_arn = module.secrets.jwt_secret_arn
  secrets_arns   = module.secrets.all_secret_arns
  kms_key_arn    = module.secrets.kms_key_arn

  log_level          = "debug"
  log_retention_days = 14
  key_storage_mode   = "memory"
  mtls_enabled       = false

  enable_container_insights = true

  tags = local.tags
}

# DNS Module
module "dns" {
  source = "../../modules/dns"

  name_prefix    = local.name_prefix
  domain_name    = var.domain_name
  api_subdomain  = "api.staging.registry"
  alb_dns_name   = module.ecs.alb_dns_name
  alb_zone_id    = module.ecs.alb_zone_id

  create_hosted_zone      = false
  create_regional_records = false
  verification_token      = "staging-verification-${random_id.verification.hex}"

  certificate_domain_validation_options = aws_acm_certificate.main.domain_validation_options

  tags = local.tags
}

resource "random_id" "verification" {
  byte_length = 16
}

# Outputs
output "api_endpoint" {
  description = "API endpoint URL"
  value       = "https://${module.dns.api_fqdn}"
}

output "grpc_endpoint" {
  description = "gRPC endpoint"
  value       = "https://${module.dns.api_fqdn}:443"
}

output "alb_dns_name" {
  description = "ALB DNS name"
  value       = module.ecs.alb_dns_name
}

output "ecr_repository_url" {
  description = "ECR repository URL"
  value       = aws_ecr_repository.registry.repository_url
}

output "cluster_name" {
  description = "ECS cluster name"
  value       = module.ecs.cluster_name
}

output "service_name" {
  description = "ECS service name"
  value       = module.ecs.service_name
}

output "db_endpoint" {
  description = "Database endpoint"
  value       = module.rds.db_endpoint
  sensitive   = true
}
