# CIRISRegistry DNS Module
# Manages Route 53 DNS records with health checks

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Data source for existing hosted zone
data "aws_route53_zone" "main" {
  count = var.create_hosted_zone ? 0 : 1
  name  = var.domain_name
}

# Create hosted zone if needed
resource "aws_route53_zone" "main" {
  count = var.create_hosted_zone ? 1 : 0
  name  = var.domain_name

  tags = var.tags
}

locals {
  zone_id = var.create_hosted_zone ? aws_route53_zone.main[0].zone_id : data.aws_route53_zone.main[0].zone_id
}

# Health check for the ALB endpoint
resource "aws_route53_health_check" "primary" {
  fqdn              = var.alb_dns_name
  port              = 443
  type              = "HTTPS"
  resource_path     = "/health"
  failure_threshold = 3
  request_interval  = 30

  regions = ["us-east-1", "us-west-2", "eu-west-1"]

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-health-check"
  })
}

# Primary API record (api.registry.ciris.ai)
resource "aws_route53_record" "api" {
  zone_id = local.zone_id
  name    = var.api_subdomain
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}

# Regional DNS records for multi-source validation
resource "aws_route53_record" "registry_us" {
  count   = var.create_regional_records ? 1 : 0
  zone_id = local.zone_id
  name    = "registry-us"
  type    = "A"

  alias {
    name                   = var.alb_dns_name
    zone_id                = var.alb_zone_id
    evaluate_target_health = true
  }
}

resource "aws_route53_record" "registry_eu" {
  count   = var.create_regional_records && var.eu_alb_dns_name != "" ? 1 : 0
  zone_id = local.zone_id
  name    = "registry-eu"
  type    = "A"

  alias {
    name                   = var.eu_alb_dns_name
    zone_id                = var.eu_alb_zone_id
    evaluate_target_health = true
  }
}

# TXT records for DNS-based verification (multi-source validation)
resource "aws_route53_record" "verification_txt" {
  zone_id = local.zone_id
  name    = "_ciris-verify"
  type    = "TXT"
  ttl     = 300
  records = [var.verification_token]
}

# HTTPS record for service discovery
resource "aws_route53_record" "https" {
  zone_id = local.zone_id
  name    = var.api_subdomain
  type    = "HTTPS"
  ttl     = 300
  records = ["1 . alpn=\"h2\" port=\"443\""]
}

# Certificate validation records
resource "aws_route53_record" "cert_validation" {
  for_each = {
    for dvo in var.certificate_domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  }

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = local.zone_id
}

# CloudWatch alarm for health check
resource "aws_cloudwatch_metric_alarm" "health_check" {
  alarm_name          = "${var.name_prefix}-health-check-failed"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 2
  metric_name         = "HealthCheckStatus"
  namespace           = "AWS/Route53"
  period              = 60
  statistic           = "Minimum"
  threshold           = 1
  alarm_description   = "CIRISRegistry health check failed"
  treat_missing_data  = "breaching"

  dimensions = {
    HealthCheckId = aws_route53_health_check.primary.id
  }

  alarm_actions = var.alarm_actions
  ok_actions    = var.ok_actions

  tags = var.tags
}
