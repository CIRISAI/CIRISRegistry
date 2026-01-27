# DNS Module Outputs

output "zone_id" {
  description = "Route 53 hosted zone ID"
  value       = local.zone_id
}

output "api_fqdn" {
  description = "Fully qualified domain name for API"
  value       = aws_route53_record.api.fqdn
}

output "health_check_id" {
  description = "Health check ID"
  value       = aws_route53_health_check.primary.id
}

output "name_servers" {
  description = "Name servers for the hosted zone"
  value       = var.create_hosted_zone ? aws_route53_zone.main[0].name_servers : null
}
