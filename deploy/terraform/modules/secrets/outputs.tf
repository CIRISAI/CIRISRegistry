# Secrets Module Outputs

output "kms_key_arn" {
  description = "KMS key ARN for secrets encryption"
  value       = aws_kms_key.secrets.arn
}

output "kms_key_id" {
  description = "KMS key ID"
  value       = aws_kms_key.secrets.key_id
}

output "db_secret_arn" {
  description = "Database credentials secret ARN"
  value       = aws_secretsmanager_secret.db_credentials.arn
}

output "db_secret_name" {
  description = "Database credentials secret name"
  value       = aws_secretsmanager_secret.db_credentials.name
}

output "jwt_secret_arn" {
  description = "JWT secret ARN"
  value       = aws_secretsmanager_secret.jwt_secret.arn
}

output "jwt_secret_name" {
  description = "JWT secret name"
  value       = aws_secretsmanager_secret.jwt_secret.name
}

output "signing_keys_secret_arn" {
  description = "Signing keys secret ARN"
  value       = aws_secretsmanager_secret.signing_keys.arn
}

output "signing_keys_secret_name" {
  description = "Signing keys secret name"
  value       = aws_secretsmanager_secret.signing_keys.name
}

output "mtls_secret_arn" {
  description = "mTLS certificates secret ARN"
  value       = var.enable_mtls ? aws_secretsmanager_secret.mtls_certs[0].arn : null
}

output "all_secret_arns" {
  description = "List of all secret ARNs for IAM policies"
  value = compact([
    aws_secretsmanager_secret.db_credentials.arn,
    aws_secretsmanager_secret.jwt_secret.arn,
    aws_secretsmanager_secret.signing_keys.arn,
    var.enable_mtls ? aws_secretsmanager_secret.mtls_certs[0].arn : null
  ])
}

output "generated_db_password" {
  description = "Generated database password"
  value       = var.use_generated_password ? random_password.db_password.result : null
  sensitive   = true
}
