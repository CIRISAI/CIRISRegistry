# CIRISRegistry Secrets Module
# Manages secrets in AWS Secrets Manager

terraform {
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
}

# KMS Key for encrypting secrets
resource "aws_kms_key" "secrets" {
  description             = "KMS key for ${var.name_prefix} secrets"
  deletion_window_in_days = 30
  enable_key_rotation     = true

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "Enable IAM User Permissions"
        Effect = "Allow"
        Principal = {
          AWS = "arn:aws:iam::${data.aws_caller_identity.current.account_id}:root"
        }
        Action   = "kms:*"
        Resource = "*"
      },
      {
        Sid    = "Allow ECS Tasks"
        Effect = "Allow"
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
        Action = [
          "kms:Decrypt",
          "kms:GenerateDataKey"
        ]
        Resource = "*"
        Condition = {
          StringEquals = {
            "kms:CallerAccount" = data.aws_caller_identity.current.account_id
          }
        }
      }
    ]
  })

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-secrets-key"
  })
}

resource "aws_kms_alias" "secrets" {
  name          = "alias/${var.name_prefix}-secrets"
  target_key_id = aws_kms_key.secrets.key_id
}

data "aws_caller_identity" "current" {}

# Generate random password for database
resource "random_password" "db_password" {
  length           = 32
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

# Generate JWT secret
resource "random_password" "jwt_secret" {
  length  = 64
  special = false
}

# Database credentials secret
resource "aws_secretsmanager_secret" "db_credentials" {
  name        = "${var.name_prefix}/database/credentials"
  description = "Database credentials for CIRISRegistry"
  kms_key_id  = aws_kms_key.secrets.arn

  recovery_window_in_days = var.environment == "production" ? 30 : 7

  tags = var.tags
}

resource "aws_secretsmanager_secret_version" "db_credentials" {
  secret_id = aws_secretsmanager_secret.db_credentials.id
  secret_string = jsonencode({
    host     = var.db_host
    port     = var.db_port
    username = var.db_username
    password = var.use_generated_password ? random_password.db_password.result : var.db_password
    dbname   = var.db_name
  })
}

# JWT secret
resource "aws_secretsmanager_secret" "jwt_secret" {
  name        = "${var.name_prefix}/jwt/secret"
  description = "JWT signing secret for CIRISRegistry"
  kms_key_id  = aws_kms_key.secrets.arn

  recovery_window_in_days = var.environment == "production" ? 30 : 7

  tags = var.tags
}

resource "aws_secretsmanager_secret_version" "jwt_secret" {
  secret_id = aws_secretsmanager_secret.jwt_secret.id
  secret_string = jsonencode({
    secret = var.use_generated_jwt ? random_password.jwt_secret.result : var.jwt_secret
  })
}

# Registry signing keys secret (for hybrid cryptography)
resource "aws_secretsmanager_secret" "signing_keys" {
  name        = "${var.name_prefix}/registry/signing-keys"
  description = "Ed25519 and ML-DSA-65 signing keys for CIRISRegistry"
  kms_key_id  = aws_kms_key.secrets.arn

  recovery_window_in_days = var.environment == "production" ? 30 : 7

  tags = var.tags
}

# Note: The actual keys should be generated and stored externally
# This is a placeholder for the secret structure
resource "aws_secretsmanager_secret_version" "signing_keys" {
  count     = var.signing_keys_json != "" ? 1 : 0
  secret_id = aws_secretsmanager_secret.signing_keys.id
  secret_string = var.signing_keys_json
}

# mTLS certificates secret (optional)
resource "aws_secretsmanager_secret" "mtls_certs" {
  count       = var.enable_mtls ? 1 : 0
  name        = "${var.name_prefix}/mtls/certificates"
  description = "mTLS certificates for CIRISRegistry"
  kms_key_id  = aws_kms_key.secrets.arn

  recovery_window_in_days = var.environment == "production" ? 30 : 7

  tags = var.tags
}

resource "aws_secretsmanager_secret_version" "mtls_certs" {
  count     = var.enable_mtls && var.mtls_certs_json != "" ? 1 : 0
  secret_id = aws_secretsmanager_secret.mtls_certs[0].id
  secret_string = var.mtls_certs_json
}

# Secret rotation (for production)
resource "aws_secretsmanager_secret_rotation" "db_credentials" {
  count               = var.enable_secret_rotation ? 1 : 0
  secret_id           = aws_secretsmanager_secret.db_credentials.id
  rotation_lambda_arn = var.rotation_lambda_arn

  rotation_rules {
    automatically_after_days = var.rotation_days
  }
}
