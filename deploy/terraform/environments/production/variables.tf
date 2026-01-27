# Production Environment Variables

variable "region" {
  description = "Primary AWS region"
  type        = string
  default     = "us-east-1"
}

variable "domain_name" {
  description = "Root domain name"
  type        = string
  default     = "ciris.ai"
}

variable "image_tag" {
  description = "Docker image tag to deploy (should be a version tag like v1.0.0)"
  type        = string
}

variable "enable_mtls" {
  description = "Enable mTLS for service-to-service communication"
  type        = bool
  default     = true
}

variable "key_storage_mode" {
  description = "Key storage mode (memory, vault, hsm)"
  type        = string
  default     = "vault"
}

variable "rotation_lambda_arn" {
  description = "ARN of Lambda function for secret rotation"
  type        = string
  default     = ""
}

variable "alarm_sns_arns" {
  description = "SNS topic ARNs for CloudWatch alarms"
  type        = list(string)
  default     = []
}
