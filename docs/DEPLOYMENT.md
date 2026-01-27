# CIRISRegistry Deployment Guide

**Version:** 1.0.0
**Last Updated:** 2026-01-26
**Target Audiences:** CIRISBridge Team, DevOps, SRE

This guide covers deploying CIRISRegistry to staging and production environments on AWS.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Prerequisites](#prerequisites)
3. [Environment Configuration](#environment-configuration)
4. [Terraform Deployment](#terraform-deployment)
5. [Ansible Configuration](#ansible-configuration)
6. [DNS and Multi-Region Setup](#dns-and-multi-region-setup)
7. [Security Considerations](#security-considerations)
8. [Monitoring and Alerting](#monitoring-and-alerting)
9. [Disaster Recovery](#disaster-recovery)
10. [Runbooks](#runbooks)

---

## Architecture Overview

### Production Architecture

```
                                    ┌─────────────────────────────────────────────────┐
                                    │              Route 53 (Global)                   │
                                    │  api.registry.ciris.ai                          │
                                    │  registry-us.ciris.ai (DNS TXT)                 │
                                    │  registry-eu.ciris.ai (DNS TXT)                 │
                                    └─────────────────────────────────────────────────┘
                                                        │
                    ┌───────────────────────────────────┼───────────────────────────────────┐
                    │                                   │                                   │
                    ▼                                   ▼                                   ▼
    ┌───────────────────────────┐   ┌───────────────────────────┐   ┌───────────────────────────┐
    │     US-EAST-1 (Primary)    │   │     US-WEST-2 (Secondary)  │   │     EU-WEST-1 (DR)        │
    │                            │   │                            │   │                            │
    │  ┌──────────────────────┐  │   │  ┌──────────────────────┐  │   │  ┌──────────────────────┐  │
    │  │    ALB (gRPC/HTTP)   │  │   │  │    ALB (gRPC/HTTP)   │  │   │  │    ALB (gRPC/HTTP)   │  │
    │  └──────────┬───────────┘  │   │  └──────────┬───────────┘  │   │  └──────────┬───────────┘  │
    │             │              │   │             │              │   │             │              │
    │  ┌──────────▼───────────┐  │   │  ┌──────────▼───────────┐  │   │  ┌──────────▼───────────┐  │
    │  │   ECS Fargate (x3)   │  │   │  │   ECS Fargate (x2)   │  │   │  │   ECS Fargate (x2)   │  │
    │  │   ciris-registry     │  │   │  │   ciris-registry     │  │   │  │   ciris-registry     │  │
    │  └──────────┬───────────┘  │   │  └──────────┬───────────┘  │   │  └──────────┬───────────┘  │
    │             │              │   │             │              │   │             │              │
    │  ┌──────────▼───────────┐  │   │  ┌──────────▼───────────┐  │   │  ┌──────────▼───────────┐  │
    │  │  RDS PostgreSQL      │  │   │  │  RDS Read Replica    │  │   │  │  RDS Read Replica    │  │
    │  │  (Multi-AZ Primary)  │──┼───┼──│  (Cross-Region)      │  │   │  │  (Cross-Region)      │  │
    │  └──────────────────────┘  │   │  └──────────────────────┘  │   │  └──────────────────────┘  │
    │                            │   │                            │   │                            │
    │  ┌──────────────────────┐  │   │                            │   │                            │
    │  │  Secrets Manager     │  │   │                            │   │                            │
    │  │  (HSM-backed keys)   │  │   │                            │   │                            │
    │  └──────────────────────┘  │   │                            │   │                            │
    └────────────────────────────┘   └────────────────────────────┘   └────────────────────────────┘
```

### Staging Architecture

```
    ┌─────────────────────────────────────────────────┐
    │              Route 53                            │
    │  api.staging.registry.ciris.ai                  │
    └─────────────────────────────────────────────────┘
                        │
                        ▼
    ┌────────────────────────────────────────────────────┐
    │              US-EAST-1 (Staging)                    │
    │                                                     │
    │  ┌──────────────────────┐  ┌──────────────────────┐│
    │  │    ALB (gRPC/HTTP)   │  │   ECS Fargate (x2)   ││
    │  └──────────┬───────────┘  │   ciris-registry     ││
    │             │              └──────────┬───────────┘│
    │             └──────────────────────────┘           │
    │                            │                       │
    │  ┌──────────────────────┐  │                       │
    │  │  RDS PostgreSQL      │◀─┘                       │
    │  │  (Single-AZ)         │                          │
    │  └──────────────────────┘                          │
    └────────────────────────────────────────────────────┘
```

---

## Prerequisites

### Tools Required

| Tool | Version | Purpose |
|------|---------|---------|
| Terraform | >= 1.6.0 | Infrastructure provisioning |
| Ansible | >= 2.15 | Configuration management |
| AWS CLI | >= 2.0 | AWS interactions |
| Docker | >= 24.0 | Container builds |
| grpcurl | >= 1.8 | gRPC testing |

### AWS Resources

- AWS Account with appropriate IAM permissions
- ECR repository for container images
- S3 bucket for Terraform state
- Route 53 hosted zone for `ciris.ai`
- ACM certificates for TLS
- KMS keys for encryption

### Required Permissions

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ec2:*",
        "ecs:*",
        "ecr:*",
        "rds:*",
        "secretsmanager:*",
        "kms:*",
        "route53:*",
        "acm:*",
        "elasticloadbalancing:*",
        "logs:*",
        "cloudwatch:*",
        "iam:PassRole",
        "iam:GetRole"
      ],
      "Resource": "*"
    }
  ]
}
```

---

## Environment Configuration

### Environment Variables

| Variable | Staging | Production | Description |
|----------|---------|------------|-------------|
| `ENVIRONMENT` | staging | production | Environment identifier |
| `GRPC_PORT` | 50051 | 50051 | gRPC server port |
| `HTTP_PORT` | 8080 | 8080 | HTTP health/metrics port |
| `RUST_LOG` | info | warn | Log level |
| `DB_HOST` | (from secrets) | (from secrets) | Database host |
| `DB_PORT` | 5432 | 5432 | Database port |
| `DB_NAME` | ciris_registry | ciris_registry | Database name |
| `DB_SSLMODE` | require | require | SSL mode |
| `JWT_SECRET` | (from secrets) | (from secrets) | JWT signing secret |
| `MTLS_ENABLED` | false | true | Enable mTLS |
| `KEY_STORAGE_MODE` | vault | hsm | Key storage backend |

### Secrets (AWS Secrets Manager)

| Secret Name | Contents |
|-------------|----------|
| `ciris/registry/{env}/db` | `{"username":"ciris","password":"...","host":"..."}` |
| `ciris/registry/{env}/jwt` | `{"secret":"..."}` |
| `ciris/registry/{env}/signing-keys` | Ed25519 + ML-DSA-65 private keys |
| `ciris/registry/{env}/tls` | TLS certificate and key |

---

## Terraform Deployment

### Directory Structure

```
deploy/terraform/
├── modules/
│   ├── vpc/           # VPC, subnets, security groups
│   ├── rds/           # PostgreSQL RDS
│   ├── ecs/           # ECS cluster, service, task
│   ├── secrets/       # Secrets Manager
│   └── dns/           # Route 53 records
├── environments/
│   ├── staging/
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   ├── outputs.tf
│   │   └── terraform.tfvars
│   └── production/
│       ├── main.tf
│       ├── variables.tf
│       ├── outputs.tf
│       └── terraform.tfvars
└── backend.tf
```

### Deploy Staging

```bash
cd deploy/terraform/environments/staging

# Initialize
terraform init -backend-config="bucket=ciris-terraform-state" \
               -backend-config="key=registry/staging/terraform.tfstate" \
               -backend-config="region=us-east-1"

# Plan
terraform plan -out=tfplan

# Apply
terraform apply tfplan
```

### Deploy Production

```bash
cd deploy/terraform/environments/production

# Initialize
terraform init -backend-config="bucket=ciris-terraform-state" \
               -backend-config="key=registry/production/terraform.tfstate" \
               -backend-config="region=us-east-1"

# Plan (with approval required)
terraform plan -out=tfplan

# Review plan carefully, then apply
terraform apply tfplan
```

### Multi-Region Production

For production, deploy to multiple regions:

```bash
# Primary (us-east-1)
cd deploy/terraform/environments/production
terraform workspace select us-east-1 || terraform workspace new us-east-1
terraform apply -var="region=us-east-1" -var="is_primary=true"

# Secondary (us-west-2)
terraform workspace select us-west-2 || terraform workspace new us-west-2
terraform apply -var="region=us-west-2" -var="is_primary=false"

# DR (eu-west-1)
terraform workspace select eu-west-1 || terraform workspace new eu-west-1
terraform apply -var="region=eu-west-1" -var="is_primary=false"
```

---

## Ansible Configuration

### Directory Structure

```
deploy/ansible/
├── site.yml                    # Main playbook
├── staging.yml                 # Staging-specific
├── production.yml              # Production-specific
├── inventories/
│   ├── staging/
│   │   └── hosts.yml
│   └── production/
│       └── hosts.yml
├── group_vars/
│   ├── all.yml
│   ├── staging.yml
│   └── production.yml
└── roles/
    ├── registry/
    ├── postgres/
    └── monitoring/
```

### Deploy to Staging

```bash
cd deploy/ansible

# Deploy registry
ansible-playbook -i inventories/staging/hosts.yml site.yml

# Deploy with specific tags
ansible-playbook -i inventories/staging/hosts.yml site.yml --tags "registry"

# Dry run
ansible-playbook -i inventories/staging/hosts.yml site.yml --check
```

### Deploy to Production

```bash
cd deploy/ansible

# Deploy with confirmation
ansible-playbook -i inventories/production/hosts.yml production.yml --check
ansible-playbook -i inventories/production/hosts.yml production.yml
```

---

## DNS and Multi-Region Setup

### DNS Records Required

| Record | Type | Value | Purpose |
|--------|------|-------|---------|
| `api.registry.ciris.ai` | A (Alias) | ALB DNS | Primary API endpoint |
| `api.staging.registry.ciris.ai` | A (Alias) | ALB DNS | Staging API |
| `registry-us.ciris.ai` | TXT | Status records | Multi-source validation (US) |
| `registry-eu.ciris.ai` | TXT | Status records | Multi-source validation (EU) |

### DNS TXT Record Format

For multi-source validation, DNS TXT records contain:

```
registry-us.ciris.ai TXT "v=ciris1 status=active version=1.1.0 sig=<base64>"
```

CIRISVerify queries multiple DNS endpoints and requires 2-of-3 agreement.

### Health-Based Routing

Configure Route 53 health checks:

```hcl
resource "aws_route53_health_check" "registry_us_east" {
  fqdn              = "api-us-east-1.registry.ciris.ai"
  port              = 443
  type              = "HTTPS"
  resource_path     = "/health"
  failure_threshold = 3
  request_interval  = 10
}
```

---

## Security Considerations

### Network Security

1. **VPC Isolation**: Registry runs in private subnets
2. **Security Groups**:
   - ALB: Ingress 443 from internet
   - ECS: Ingress 50051, 8080 from ALB only
   - RDS: Ingress 5432 from ECS only
3. **NACLs**: Default deny with explicit allows

### Encryption

| Data | Method |
|------|--------|
| Data at rest (RDS) | AWS KMS (CMK) |
| Data at rest (Secrets) | AWS KMS (CMK) |
| Data in transit (external) | TLS 1.3 |
| Data in transit (internal) | mTLS |
| Signing keys | HSM-backed (production) |

### mTLS Configuration

Production requires mTLS for CIRISPortal connections:

```yaml
# group_vars/production.yml
mtls_enabled: true
tls_cert_path: /etc/ciris/tls/server.crt
tls_key_path: /etc/ciris/tls/server.key
ca_cert_path: /etc/ciris/tls/ca.crt
```

### Key Management

| Environment | Key Storage | Notes |
|-------------|-------------|-------|
| Development | Memory | Ephemeral, regenerated on restart |
| Staging | HashiCorp Vault | Centralized secret management |
| Production | AWS CloudHSM | FIPS 140-2 Level 3 compliant |

---

## Monitoring and Alerting

### Metrics Exposed

The `/metrics` endpoint exposes Prometheus metrics:

```
ciris_registry_info{version="1.1.0"} 1
ciris_registry_uptime_seconds 3600
ciris_registry_grpc_requests_total{method="LookupAgent",status="ok"} 1234
ciris_registry_grpc_latency_seconds{method="LookupAgent",quantile="0.99"} 0.05
ciris_registry_db_connections_active 5
ciris_registry_db_connections_max 20
```

### CloudWatch Alarms

| Alarm | Threshold | Action |
|-------|-----------|--------|
| HealthCheckFailed | 3 consecutive failures | PagerDuty |
| HighLatency | p99 > 500ms for 5 min | Slack |
| HighErrorRate | > 1% errors for 5 min | PagerDuty |
| DBConnections | > 80% pool used | Slack |
| DiskSpace | < 20% free | Slack |

### Logging

Logs are sent to CloudWatch Logs:

| Log Group | Contents |
|-----------|----------|
| `/ciris/registry/{env}/application` | Application logs (JSON) |
| `/ciris/registry/{env}/access` | Access logs |
| `/ciris/registry/{env}/audit` | Audit events |

---

## Disaster Recovery

### RTO/RPO Targets

| Environment | RTO | RPO |
|-------------|-----|-----|
| Staging | 4 hours | 1 hour |
| Production | 15 minutes | 5 minutes |

### Backup Strategy

| Data | Frequency | Retention | Location |
|------|-----------|-----------|----------|
| RDS Snapshots | Hourly | 7 days | Same region |
| RDS Snapshots | Daily | 30 days | Cross-region |
| Signing Keys | On rotation | Forever | S3 (encrypted) |
| Audit Logs | Continuous | 7 years | S3 Glacier |

### Failover Procedure

1. **Automatic**: Route 53 health checks trigger DNS failover
2. **Manual**: Promote read replica to primary

```bash
# Promote RDS read replica (manual failover)
aws rds promote-read-replica \
  --db-instance-identifier ciris-registry-us-west-2 \
  --region us-west-2

# Update Terraform state
terraform import aws_db_instance.primary <new-primary-id>
```

---

## Runbooks

### Deploy New Version

```bash
# 1. Build and push container
docker build -t ciris-registry:v1.2.0 .
docker tag ciris-registry:v1.2.0 <account>.dkr.ecr.us-east-1.amazonaws.com/ciris-registry:v1.2.0
docker push <account>.dkr.ecr.us-east-1.amazonaws.com/ciris-registry:v1.2.0

# 2. Update task definition
cd deploy/terraform/environments/staging
terraform apply -var="image_tag=v1.2.0"

# 3. Force new deployment
aws ecs update-service \
  --cluster ciris-registry-staging \
  --service ciris-registry \
  --force-new-deployment
```

### Rotate Signing Keys

```bash
# 1. Generate new key pair (done via Admin API)
grpcurl -plaintext -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{"new_key_id": "key-2026-02", "target_storage": 2}' \
  api.registry.ciris.ai:443 \
  ciris.registry.v1.RegistryAdminService/RotateSigningKey

# 2. Monitor dual-signing period (24 hours default)
# 3. Old key automatically retired after grace period
```

### Emergency Shutdown

```bash
# Enable emergency lockdown
grpcurl -plaintext -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{
    "reason": "Security incident",
    "severity": 3,
    "lock_duration_seconds": 3600,
    "allowed_operations": ["health_check", "get_emergency_status"]
  }' \
  api.registry.ciris.ai:443 \
  ciris.registry.v1.RegistryAdminService/SetEmergencyShutdown

# Clear emergency (after incident resolved)
grpcurl -plaintext -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{"reason": "Incident resolved"}' \
  api.registry.ciris.ai:443 \
  ciris.registry.v1.RegistryAdminService/ClearEmergencyShutdown
```

### Database Migration

```bash
# Migrations run automatically on startup
# For manual migration:
docker run --rm \
  -e DATABASE_URL="postgres://..." \
  ciris-registry:v1.2.0 \
  /app/ciris-registry migrate
```

---

## Contacts

| Role | Contact | Escalation |
|------|---------|------------|
| On-call SRE | sre@ciris.ai | PagerDuty |
| Security | security@ciris.ai | Emergency hotline |
| CIRISBridge Lead | bridge@ciris.ai | Slack #ciris-bridge |

---

*Document maintained by CIRISBridge Team*
