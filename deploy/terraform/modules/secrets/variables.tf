# Secrets Module Variables

variable "name_prefix" {
  description = "Prefix for resource names"
  type        = string
}

variable "environment" {
  description = "Environment name (staging, production)"
  type        = string
}

# Database configuration
variable "db_host" {
  description = "Database hostname"
  type        = string
}

variable "db_port" {
  description = "Database port"
  type        = number
  default     = 5432
}

variable "db_username" {
  description = "Database username"
  type        = string
  default     = "ciris_admin"
}

variable "db_password" {
  description = "Database password (if not using generated)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "db_name" {
  description = "Database name"
  type        = string
  default     = "ciris_registry"
}

variable "use_generated_password" {
  description = "Use auto-generated database password"
  type        = bool
  default     = true
}

# JWT configuration
variable "jwt_secret" {
  description = "JWT secret (if not using generated)"
  type        = string
  default     = ""
  sensitive   = true
}

variable "use_generated_jwt" {
  description = "Use auto-generated JWT secret"
  type        = bool
  default     = true
}

# Signing keys
variable "signing_keys_json" {
  description = "JSON containing signing keys (Ed25519 and ML-DSA-65)"
  type        = string
  default     = ""
  sensitive   = true
}

# mTLS configuration
variable "enable_mtls" {
  description = "Enable mTLS secrets"
  type        = bool
  default     = false
}

variable "mtls_certs_json" {
  description = "JSON containing mTLS certificates"
  type        = string
  default     = ""
  sensitive   = true
}

# Secret rotation
variable "enable_secret_rotation" {
  description = "Enable automatic secret rotation"
  type        = bool
  default     = false
}

variable "rotation_lambda_arn" {
  description = "ARN of the Lambda function for secret rotation"
  type        = string
  default     = ""
}

variable "rotation_days" {
  description = "Number of days between automatic secret rotations"
  type        = number
  default     = 30
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}
