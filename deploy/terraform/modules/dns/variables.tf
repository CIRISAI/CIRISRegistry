# DNS Module Variables

variable "name_prefix" {
  description = "Prefix for resource names"
  type        = string
}

variable "domain_name" {
  description = "Root domain name (e.g., ciris.ai)"
  type        = string
}

variable "api_subdomain" {
  description = "API subdomain (e.g., api.registry)"
  type        = string
  default     = "api.registry"
}

variable "create_hosted_zone" {
  description = "Create a new hosted zone (false to use existing)"
  type        = bool
  default     = false
}

variable "alb_dns_name" {
  description = "ALB DNS name"
  type        = string
}

variable "alb_zone_id" {
  description = "ALB Route 53 zone ID"
  type        = string
}

variable "create_regional_records" {
  description = "Create regional DNS records for multi-source validation"
  type        = bool
  default     = true
}

variable "eu_alb_dns_name" {
  description = "EU region ALB DNS name (for multi-region)"
  type        = string
  default     = ""
}

variable "eu_alb_zone_id" {
  description = "EU region ALB zone ID"
  type        = string
  default     = ""
}

variable "verification_token" {
  description = "Verification token for DNS-based validation"
  type        = string
  default     = "ciris-registry-verification-token"
}

variable "certificate_domain_validation_options" {
  description = "Domain validation options from ACM certificate"
  type = list(object({
    domain_name           = string
    resource_record_name  = string
    resource_record_type  = string
    resource_record_value = string
  }))
  default = []
}

variable "alarm_actions" {
  description = "SNS topic ARNs for alarm notifications"
  type        = list(string)
  default     = []
}

variable "ok_actions" {
  description = "SNS topic ARNs for OK notifications"
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Tags to apply to resources"
  type        = map(string)
  default     = {}
}
