# CIRISRegistry - Infrastructure Configuration
#
# Deploys the CIRISRegistry API across multiple regions for:
# - High availability
# - Multi-source validation (2-of-3 agreement)
# - Geographic redundancy
#
# Architecture:
#   ┌─────────────────────────────────────────────────────────────┐
#   │                    Cloudflare DNS                            │
#   │  api.registry.ciris.ai → Load Balancer                      │
#   │  registry-us.ciris.ai  → US Region (DNS TXT)                │
#   │  registry-eu.ciris.ai  → EU Region (DNS TXT)                │
#   └─────────────────────────────────────────────────────────────┘
#                              │
#          ┌───────────────────┼───────────────────┐
#          ▼                   ▼                   ▼
#   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
#   │ US Region   │    │ EU Region   │    │ Backup      │
#   │ (Vultr)     │    │ (Hetzner)   │    │ (Optional)  │
#   │             │    │             │    │             │
#   │ API + DB    │◄──►│ API + DB    │◄──►│ API + DB    │
#   │ (Primary)   │    │ (Replica)   │    │ (Replica)   │
#   └─────────────┘    └─────────────┘    └─────────────┘
#
# Usage:
#   terraform init
#   terraform plan -var-file=secrets.tfvars
#   terraform apply -var-file=secrets.tfvars

terraform {
  required_version = ">= 1.0"

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
    vultr = {
      source  = "vultr/vultr"
      version = "~> 2.0"
    }
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.45"
    }
  }

  # Backend configuration for CIRISBridge
  # backend "s3" {
  #   bucket         = "ciris-terraform-state"
  #   key            = "cirisregistry/terraform.tfstate"
  #   region         = "us-east-1"
  #   encrypt        = true
  #   dynamodb_table = "ciris-terraform-locks"
  # }
}

# =============================================================================
# PROVIDERS
# =============================================================================

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

provider "vultr" {
  api_key = var.vultr_api_key
}

provider "hcloud" {
  token = var.hetzner_api_token
}

# =============================================================================
# VARIABLES
# =============================================================================

variable "cloudflare_api_token" {
  description = "Cloudflare API token"
  type        = string
  sensitive   = true
}

variable "cloudflare_zone_id" {
  description = "Cloudflare zone ID for ciris.ai"
  type        = string
}

variable "vultr_api_key" {
  description = "Vultr API key"
  type        = string
  sensitive   = true
}

variable "hetzner_api_token" {
  description = "Hetzner Cloud API token"
  type        = string
  sensitive   = true
}

variable "environment" {
  description = "Environment (production, staging)"
  type        = string
  default     = "production"
}

variable "ssh_public_key" {
  description = "SSH public key for server access"
  type        = string
}

variable "db_password" {
  description = "PostgreSQL database password"
  type        = string
  sensitive   = true
}

variable "registry_signing_key" {
  description = "Ed25519 private key for signing registry records (base64)"
  type        = string
  sensitive   = true
}

variable "registry_signing_key_pq" {
  description = "ML-DSA-65 private key for signing registry records (base64)"
  type        = string
  sensitive   = true
}

# =============================================================================
# SSH KEYS
# =============================================================================

resource "vultr_ssh_key" "ciris" {
  name    = "ciris-registry-${var.environment}"
  ssh_key = var.ssh_public_key
}

resource "hcloud_ssh_key" "ciris" {
  name       = "ciris-registry-${var.environment}"
  public_key = var.ssh_public_key
}

# =============================================================================
# US REGION (Vultr - New Jersey)
# =============================================================================

resource "vultr_instance" "registry_us" {
  label             = "ciris-registry-us-${var.environment}"
  region            = "ewr"  # New Jersey
  plan              = "vc2-2c-4gb"  # 2 vCPU, 4GB RAM
  os_id             = 1743  # Ubuntu 22.04
  ssh_key_ids       = [vultr_ssh_key.ciris.id]
  enable_ipv6       = true
  backups           = "enabled"
  ddos_protection   = true
  activation_email  = false

  tags = ["ciris", "registry", var.environment, "us"]
}

# =============================================================================
# EU REGION (Hetzner - Germany)
# =============================================================================

resource "hcloud_server" "registry_eu" {
  name        = "ciris-registry-eu-${var.environment}"
  server_type = "cx21"  # 2 vCPU, 4GB RAM
  image       = "ubuntu-22.04"
  location    = "nbg1"  # Nuremberg
  ssh_keys    = [hcloud_ssh_key.ciris.id]
  backups     = true

  labels = {
    project     = "ciris"
    component   = "registry"
    environment = var.environment
    region      = "eu"
  }
}

# Hetzner Firewall
resource "hcloud_firewall" "registry" {
  name = "ciris-registry-${var.environment}"

  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "22"
    source_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "80"
    source_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "443"
    source_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "5432"  # PostgreSQL replication
    source_ips = [
      "${vultr_instance.registry_us.main_ip}/32"
    ]
  }
}

resource "hcloud_firewall_attachment" "registry_eu" {
  firewall_id = hcloud_firewall.registry.id
  server_ids  = [hcloud_server.registry_eu.id]
}

# =============================================================================
# CLOUDFLARE DNS
# =============================================================================

# Main API endpoint (load balanced)
resource "cloudflare_record" "api_registry_us" {
  zone_id = var.cloudflare_zone_id
  name    = "api.registry"
  type    = "A"
  content = vultr_instance.registry_us.main_ip
  proxied = true
  comment = "CIRISRegistry API - US endpoint"
}

resource "cloudflare_record" "api_registry_eu" {
  zone_id = var.cloudflare_zone_id
  name    = "api.registry"
  type    = "A"
  content = hcloud_server.registry_eu.ipv4_address
  proxied = true
  comment = "CIRISRegistry API - EU endpoint"
}

# US DNS endpoint for multi-source validation
resource "cloudflare_record" "registry_us" {
  zone_id = var.cloudflare_zone_id
  name    = "registry-us"
  type    = "A"
  content = vultr_instance.registry_us.main_ip
  proxied = false  # Direct access for DNS TXT queries
  comment = "CIRISRegistry US - Multi-source validation"
}

# EU DNS endpoint for multi-source validation
resource "cloudflare_record" "registry_eu" {
  zone_id = var.cloudflare_zone_id
  name    = "registry-eu"
  type    = "A"
  content = hcloud_server.registry_eu.ipv4_address
  proxied = false  # Direct access for DNS TXT queries
  comment = "CIRISRegistry EU - Multi-source validation"
}

# =============================================================================
# CLOUDFLARE LOAD BALANCER (Optional - for api.registry.ciris.ai)
# =============================================================================

resource "cloudflare_load_balancer_pool" "registry" {
  account_id = var.cloudflare_account_id
  name       = "ciris-registry-${var.environment}"

  origins {
    name    = "us"
    address = vultr_instance.registry_us.main_ip
    enabled = true
    weight  = 1
  }

  origins {
    name    = "eu"
    address = hcloud_server.registry_eu.ipv4_address
    enabled = true
    weight  = 1
  }

  minimum_origins = 1

  monitor = cloudflare_load_balancer_monitor.registry.id
}

resource "cloudflare_load_balancer_monitor" "registry" {
  account_id     = var.cloudflare_account_id
  type           = "https"
  expected_codes = "200"
  method         = "GET"
  path           = "/health"
  interval       = 60
  timeout        = 5
  retries        = 2
}

variable "cloudflare_account_id" {
  description = "Cloudflare account ID"
  type        = string
}

# =============================================================================
# OUTPUTS
# =============================================================================

output "us_server_ip" {
  description = "US region server IP"
  value       = vultr_instance.registry_us.main_ip
}

output "eu_server_ip" {
  description = "EU region server IP"
  value       = hcloud_server.registry_eu.ipv4_address
}

output "api_endpoint" {
  description = "Main API endpoint"
  value       = "https://api.registry.ciris.ai"
}

output "us_dns_endpoint" {
  description = "US DNS endpoint for multi-source validation"
  value       = "registry-us.ciris.ai"
}

output "eu_dns_endpoint" {
  description = "EU DNS endpoint for multi-source validation"
  value       = "registry-eu.ciris.ai"
}
