# CIRISRegistry Production Environment
# Terraform configuration for production deployment

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
    key            = "registry/production/terraform.tfstate"
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
      Environment = "production"
      ManagedBy   = "Terraform"
      Team        = "CIRISBridge"
    }
  }
}

# Secondary provider for EU region (multi-region deployment)
provider "aws" {
  alias  = "eu"
  region = "eu-west-1"

  default_tags {
    tags = {
      Project     = "CIRISRegistry"
      Environment = "production"
      ManagedBy   = "Terraform"
      Team        = "CIRISBridge"
      Region      = "eu-west-1"
    }
  }
}

locals {
  name_prefix = "ciris-registry-prod"
  environment = "production"

  us_availability_zones = [
    "${var.region}a",
    "${var.region}b",
    "${var.region}c"
  ]

  eu_availability_zones = [
    "eu-west-1a",
    "eu-west-1b",
    "eu-west-1c"
  ]

  tags = {
    Environment = local.environment
    Service     = "CIRISRegistry"
    Criticality = "high"
  }
}

# ============================================
# US-EAST-1 PRIMARY REGION
# ============================================

# VPC Module - US
module "vpc_us" {
  source = "../../modules/vpc"

  name_prefix          = "${local.name_prefix}-us"
  region               = var.region
  vpc_cidr             = "10.20.0.0/16"
  availability_zones   = local.us_availability_zones
  enable_nat_gateway   = true
  enable_vpc_endpoints = true
  tags                 = local.tags
}

# Secrets Module - US
module "secrets_us" {
  source = "../../modules/secrets"

  name_prefix            = "${local.name_prefix}-us"
  environment            = local.environment
  db_host                = module.rds_us.db_host
  db_port                = 5432
  db_username            = "ciris_admin"
  db_name                = "ciris_registry"
  use_generated_password = true
  use_generated_jwt      = true
  enable_mtls            = var.enable_mtls
  enable_secret_rotation = true
  rotation_lambda_arn    = var.rotation_lambda_arn
  rotation_days          = 30
  tags                   = local.tags
}

# RDS Module - US (Primary)
module "rds_us" {
  source = "../../modules/rds"

  name_prefix         = "${local.name_prefix}-us"
  environment         = local.environment
  database_subnet_ids = module.vpc_us.database_subnet_ids
  security_group_id   = module.vpc_us.rds_security_group_id
  kms_key_arn         = module.secrets_us.kms_key_arn

  instance_class        = "db.r6g.large"
  allocated_storage     = 100
  max_allocated_storage = 500
  postgres_version      = "15.4"

  db_password = module.secrets_us.generated_db_password
  multi_az    = true

  enable_performance_insights = true
  enable_enhanced_monitoring  = true
  backup_retention_days       = 35
  create_read_replica         = true
  replica_instance_class      = "db.r6g.large"

  tags = local.tags
}

# ACM Certificate - US
resource "aws_acm_certificate" "main_us" {
  domain_name = "registry.${var.domain_name}"
  subject_alternative_names = [
    "api.registry.${var.domain_name}",
    "registry-us.${var.domain_name}"
  ]
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }

  tags = local.tags
}

# ECR Repository - US
resource "aws_ecr_repository" "registry_us" {
  name                 = "ciris-registry"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  encryption_configuration {
    encryption_type = "KMS"
    kms_key         = module.secrets_us.kms_key_arn
  }

  tags = local.tags
}

# ECR Lifecycle Policy
resource "aws_ecr_lifecycle_policy" "registry_us" {
  repository = aws_ecr_repository.registry_us.name

  policy = jsonencode({
    rules = [
      {
        rulePriority = 1
        description  = "Keep last 30 production images"
        selection = {
          tagStatus     = "tagged"
          tagPrefixList = ["v"]
          countType     = "imageCountMoreThan"
          countNumber   = 30
        }
        action = {
          type = "expire"
        }
      },
      {
        rulePriority = 2
        description  = "Expire untagged images after 7 days"
        selection = {
          tagStatus   = "untagged"
          countType   = "sinceImagePushed"
          countUnit   = "days"
          countNumber = 7
        }
        action = {
          type = "expire"
        }
      }
    ]
  })
}

# ECS Module - US
module "ecs_us" {
  source = "../../modules/ecs"

  name_prefix           = "${local.name_prefix}-us"
  environment           = local.environment
  region                = var.region
  vpc_id                = module.vpc_us.vpc_id
  public_subnet_ids     = module.vpc_us.public_subnet_ids
  private_subnet_ids    = module.vpc_us.private_subnet_ids
  alb_security_group_id = module.vpc_us.alb_security_group_id
  ecs_security_group_id = module.vpc_us.ecs_security_group_id

  ecr_repository_url = aws_ecr_repository.registry_us.repository_url
  image_tag          = var.image_tag
  certificate_arn    = aws_acm_certificate.main_us.arn

  task_cpu      = 1024
  task_memory   = 2048
  desired_count = 3
  min_capacity  = 2
  max_capacity  = 20

  db_secret_arn  = module.secrets_us.db_secret_arn
  jwt_secret_arn = module.secrets_us.jwt_secret_arn
  secrets_arns   = module.secrets_us.all_secret_arns
  kms_key_arn    = module.secrets_us.kms_key_arn

  log_level          = "info"
  log_retention_days = 90
  key_storage_mode   = var.key_storage_mode
  mtls_enabled       = var.enable_mtls

  enable_container_insights = true

  tags = local.tags
}

# ============================================
# EU-WEST-1 SECONDARY REGION
# ============================================

# VPC Module - EU
module "vpc_eu" {
  source = "../../modules/vpc"
  providers = {
    aws = aws.eu
  }

  name_prefix          = "${local.name_prefix}-eu"
  region               = "eu-west-1"
  vpc_cidr             = "10.30.0.0/16"
  availability_zones   = local.eu_availability_zones
  enable_nat_gateway   = true
  enable_vpc_endpoints = true
  tags                 = local.tags
}

# Secrets Module - EU
module "secrets_eu" {
  source = "../../modules/secrets"
  providers = {
    aws = aws.eu
  }

  name_prefix            = "${local.name_prefix}-eu"
  environment            = local.environment
  db_host                = module.rds_eu.db_host
  db_port                = 5432
  db_username            = "ciris_admin"
  db_name                = "ciris_registry"
  use_generated_password = true
  use_generated_jwt      = true
  enable_mtls            = var.enable_mtls
  tags                   = local.tags
}

# RDS Module - EU (Read Replica or Standalone)
module "rds_eu" {
  source = "../../modules/rds"
  providers = {
    aws = aws.eu
  }

  name_prefix         = "${local.name_prefix}-eu"
  environment         = local.environment
  database_subnet_ids = module.vpc_eu.database_subnet_ids
  security_group_id   = module.vpc_eu.rds_security_group_id
  kms_key_arn         = module.secrets_eu.kms_key_arn

  instance_class        = "db.r6g.large"
  allocated_storage     = 100
  max_allocated_storage = 500
  postgres_version      = "15.4"

  db_password = module.secrets_eu.generated_db_password
  multi_az    = true

  enable_performance_insights = true
  enable_enhanced_monitoring  = true
  backup_retention_days       = 35

  tags = local.tags
}

# ACM Certificate - EU
resource "aws_acm_certificate" "main_eu" {
  provider    = aws.eu
  domain_name = "registry-eu.${var.domain_name}"
  subject_alternative_names = [
    "api.registry-eu.${var.domain_name}"
  ]
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }

  tags = local.tags
}

# ECR Repository - EU (Cross-region replication from US)
resource "aws_ecr_repository" "registry_eu" {
  provider             = aws.eu
  name                 = "ciris-registry"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  encryption_configuration {
    encryption_type = "KMS"
    kms_key         = module.secrets_eu.kms_key_arn
  }

  tags = local.tags
}

# ECS Module - EU
module "ecs_eu" {
  source = "../../modules/ecs"
  providers = {
    aws = aws.eu
  }

  name_prefix           = "${local.name_prefix}-eu"
  environment           = local.environment
  region                = "eu-west-1"
  vpc_id                = module.vpc_eu.vpc_id
  public_subnet_ids     = module.vpc_eu.public_subnet_ids
  private_subnet_ids    = module.vpc_eu.private_subnet_ids
  alb_security_group_id = module.vpc_eu.alb_security_group_id
  ecs_security_group_id = module.vpc_eu.ecs_security_group_id

  ecr_repository_url = aws_ecr_repository.registry_eu.repository_url
  image_tag          = var.image_tag
  certificate_arn    = aws_acm_certificate.main_eu.arn

  task_cpu      = 1024
  task_memory   = 2048
  desired_count = 2
  min_capacity  = 2
  max_capacity  = 10

  db_secret_arn  = module.secrets_eu.db_secret_arn
  jwt_secret_arn = module.secrets_eu.jwt_secret_arn
  secrets_arns   = module.secrets_eu.all_secret_arns
  kms_key_arn    = module.secrets_eu.kms_key_arn

  log_level          = "info"
  log_retention_days = 90
  key_storage_mode   = var.key_storage_mode
  mtls_enabled       = var.enable_mtls

  enable_container_insights = true

  tags = local.tags
}

# ============================================
# GLOBAL DNS
# ============================================

# DNS Module with multi-region support
module "dns" {
  source = "../../modules/dns"

  name_prefix    = local.name_prefix
  domain_name    = var.domain_name
  api_subdomain  = "api.registry"
  alb_dns_name   = module.ecs_us.alb_dns_name
  alb_zone_id    = module.ecs_us.alb_zone_id

  create_hosted_zone      = false
  create_regional_records = true
  eu_alb_dns_name         = module.ecs_eu.alb_dns_name
  eu_alb_zone_id          = module.ecs_eu.alb_zone_id

  verification_token = "production-verification-${random_id.verification.hex}"

  certificate_domain_validation_options = aws_acm_certificate.main_us.domain_validation_options

  alarm_actions = var.alarm_sns_arns
  ok_actions    = var.alarm_sns_arns

  tags = local.tags
}

resource "random_id" "verification" {
  byte_length = 16
}

# ============================================
# OUTPUTS
# ============================================

output "api_endpoint_us" {
  description = "US API endpoint URL"
  value       = "https://api.registry.${var.domain_name}"
}

output "api_endpoint_eu" {
  description = "EU API endpoint URL"
  value       = "https://registry-eu.${var.domain_name}"
}

output "alb_dns_name_us" {
  description = "US ALB DNS name"
  value       = module.ecs_us.alb_dns_name
}

output "alb_dns_name_eu" {
  description = "EU ALB DNS name"
  value       = module.ecs_eu.alb_dns_name
}

output "ecr_repository_url_us" {
  description = "US ECR repository URL"
  value       = aws_ecr_repository.registry_us.repository_url
}

output "ecr_repository_url_eu" {
  description = "EU ECR repository URL"
  value       = aws_ecr_repository.registry_eu.repository_url
}

output "cluster_name_us" {
  description = "US ECS cluster name"
  value       = module.ecs_us.cluster_name
}

output "cluster_name_eu" {
  description = "EU ECS cluster name"
  value       = module.ecs_eu.cluster_name
}

output "db_endpoint_us" {
  description = "US Database endpoint"
  value       = module.rds_us.db_endpoint
  sensitive   = true
}

output "db_endpoint_eu" {
  description = "EU Database endpoint"
  value       = module.rds_eu.db_endpoint
  sensitive   = true
}
