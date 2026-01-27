# UIUX-001: CIRISPortal Screen Guide

**Version:** 2.0.0
**Status:** Draft
**Authors:** CIRIS L3C
**Date:** 2026-01-26

This document provides guidance for the CIRISPortal team to design screens that consume the CIRISRegistry v1.1.0 API.

---

## Table of Contents

1. [Screen Architecture Overview](#screen-architecture-overview)
2. [Public Screens](#public-screens)
3. [Authenticated Screens](#authenticated-screens)
4. [Admin Screens](#admin-screens)
5. [Error Handling Patterns](#error-handling-patterns)
6. [State Management](#state-management)
7. [Accessibility Requirements](#accessibility-requirements)

---

## Screen Architecture Overview

```
CIRISPortal
├── Public (No Auth)
│   ├── Health Status Page
│   ├── Agent Verification (Public Lookup)
│   └── Emergency Status Banner
│
├── Authenticated (OAuth)
│   ├── Dashboard
│   ├── Organization Management
│   │   ├── Organization Settings
│   │   ├── User Management
│   │   └── Billing & License
│   │
│   ├── Key Management
│   │   ├── Key List
│   │   ├── Key Details
│   │   ├── Key Rotation
│   │   └── Key Escrow
│   │
│   ├── Audit & Compliance
│   │   ├── Audit Log Viewer
│   │   ├── Compliance Reports
│   │   └── Export Tools
│   │
│   └── Settings
│       ├── Profile
│       ├── Security (MFA)
│       └── Webhooks
│
└── Admin (Elevated)
    ├── Partner Management
    │   ├── Partner List
    │   ├── Partner Detail
    │   ├── License Expiration
    │   └── Partner Activity
    │
    ├── Agent Registry
    │   ├── Agent List
    │   ├── Agent Registration
    │   ├── Build Attestations
    │   └── Batch Operations
    │
    ├── Incident Response
    │   ├── Emergency Controls
    │   ├── Mass Revocation
    │   └── Revocation List
    │
    ├── Registry Health
    │   ├── Health Dashboard
    │   ├── Signing Keys
    │   └── HSM Management
    │
    └── System Settings
        ├── Webhooks (Global)
        └── Test Data Cleanup
```

---

## Public Screens

### 1. Health Status Page

**Purpose:** Public system status visibility.

**API Calls:**
- `RegistryService/HealthCheck` (include_diagnostics=true)
- `RegistryService/GetEmergencyStatus`

**Components:**

| Component | Data Source | Refresh |
|-----------|-------------|---------|
| Overall Status | HealthCheckResponse.status | 30 sec |
| Component Health | HealthCheckResponse.components | 30 sec |
| Version Info | server_version, build_commit | Static |
| Emergency Banner | EmergencyStatusResponse | 10 sec |

**Status Indicators:**
- SERVING: Green checkmark
- NOT_SERVING: Red X
- UNKNOWN: Gray question mark

### 2. Agent Verification (Public)

**Purpose:** Allow anyone to verify an agent hash.

**API Calls:**
- `RegistryService/LookupAgent`

**Form:**
| Field | Type | Validation |
|-------|------|------------|
| Agent Hash | Text | 64-char hex or 44-char base64 |

**Result Display:**
- **Found:** Agent type, version, status, capabilities summary
- **Not Found:** "Agent not registered" message

---

## Authenticated Screens

### 3. Dashboard

**Purpose:** At-a-glance status for organization health.

**API Calls:**
- `PortalService/GetOrganization` - Organization details
- `RegistryService/LookupPartner` - License status
- `PortalService/ListKeys` - Active key count
- `RegistryAdminService/GetPartnerActivity` - Health assessment
- `RegistryAdminService/ListExpiringLicenses` - Expiration warnings

**Key Components:**

| Component | Data Source | Refresh |
|-----------|-------------|---------|
| License Status Card | PartnerRecord.status, expires_at | 5 min |
| Key Health Widget | PartnerKeyRecord.status, days since rotation | 5 min |
| Activity Summary | PartnerActivityResponse | 15 min |
| Expiration Alert Banner | ListExpiringLicensesResponse | 1 hr |
| Quick Actions | Static | N/A |

**Visual States:**
- **HEALTHY (Green):** Active license, key rotated < 90 days, active lookups
- **WARNING (Yellow):** License expiring < 30 days, key > 90 days old, idle partner
- **CRITICAL (Red):** License expired/suspended, key revoked, inactive > 30 days

**Quick Actions:**
- Generate New Key
- View Audit Log
- Download Compliance Report
- Rotate Key

**UX Notes:**
- Show `days_remaining` prominently if < 90
- Link "Renew License" to billing flow
- Show `recommendations` from health assessment

---

### 4. Organization Settings

**Purpose:** Manage organization profile and contacts.

**API Calls:**
- `PortalService/GetOrganization`
- `PortalService/UpdateOrganization`

**Form Fields:**

| Field | Type | Validation | Notes |
|-------|------|------------|-------|
| name | Text | Required, 2-100 chars | Display name |
| legal_name | Text | Required | Used for invoicing |
| tax_id | Text | Country-specific format | EIN, VAT, etc. |
| primary_email | Email | Required, verified | Main contact |
| billing_email | Email | Required | Invoice recipient |
| technical_contact_email | Email | Required | For incidents |
| compliance_contact_email | Email | Optional | For audits |
| oauth_domain | Text | Read-only | Set during onboarding |

**Actions:**
- Save Changes (requires ORG_ADMIN role)
- View Partner License (links to Billing)

**Error Handling:**
- `REGISTRY_ERROR_ORG_NOT_FOUND` → Redirect to login
- `REGISTRY_ERROR_INSUFFICIENT_ROLE` → Show "Contact admin" message

---

### 5. User Management

**Purpose:** Invite, manage, and deactivate organization users.

**API Calls:**
- `PortalService/ListOrgUsers`
- `PortalService/CreateOrgUser`
- `PortalService/UpdateOrgUser`
- `PortalService/BatchCreateOrgUsers` (bulk import)

**List View Columns:**

| Column | Field | Sortable | Filterable |
|--------|-------|----------|------------|
| Name | name | Yes | Yes |
| Email | email | Yes | Yes |
| Role | role | Yes | Yes |
| MFA | mfa_enabled | Yes | Yes |
| Status | active | Yes | Yes |
| Last Login | last_login_at | Yes | No |

**Role Selector:**
| Role | Display Name | Capabilities |
|------|--------------|--------------|
| ORG_ADMIN | Administrator | Full access |
| ORG_KEY_MANAGER | Key Manager | Key ops, no user management |
| ORG_OPERATOR | Operator | View + limited actions |
| ORG_VIEWER | Viewer | Read-only |

**Bulk Import:**
- CSV upload with columns: email, name, role
- Uses `BatchCreateOrgUsers` with `BATCH_BEST_EFFORT`
- Show results: successful count, failed rows with errors
- Download error report

**UX Notes:**
- Show MFA badge (shield icon) for users with MFA enabled
- Require confirmation for role changes to/from ORG_ADMIN
- "Last Login: Never" if `last_login_at` is 0

---

### 6. Key Management - List View

**Purpose:** View all keys and their lifecycle status.

**API Calls:**
- `PortalService/ListKeys`

**List View Columns:**

| Column | Field | Visual |
|--------|-------|--------|
| Key ID | key_id (truncated) | Monospace, copy button |
| Fingerprint | ed25519_fingerprint (first 16 chars) | Monospace |
| Status | status | Badge (color-coded) |
| Created | created_at | Relative time |
| Activated | activated_at | Relative time or "Pending" |
| Grace Period | grace_period_expires_at | Countdown if set |

**Status Badges:**
| Status | Color | Icon |
|--------|-------|------|
| KEY_ACTIVE | Green | Checkmark |
| KEY_PENDING | Yellow | Clock |
| KEY_ROTATED | Blue | Refresh |
| KEY_REVOKED | Red | X |
| KEY_ESCROWED | Purple | Lock |

**Actions:**
- Generate New Key (ORG_KEY_MANAGER+)
- Activate (for PENDING keys)
- Rotate (for ACTIVE keys)
- Revoke (with confirmation)
- View Details

---

### 7. Key Management - Generate Key

**Purpose:** Generate new hybrid cryptographic keypair.

**API Calls:**
- `PortalService/GenerateKeyPair`

**Form:**
| Field | Type | Options |
|-------|------|---------|
| Activate Immediately | Toggle | Yes/No |

**Result Display:**
- Key ID
- Ed25519 Fingerprint (full, copyable)
- ML-DSA-65 Fingerprint (full, copyable)
- Status (ACTIVE or PENDING)
- Creation timestamp

**Post-Generation Actions:**
- Download public key bundle
- Copy fingerprints
- View in key list

---

### 8. Key Management - Rotation Flow

**Purpose:** Guide user through key rotation with minimal downtime.

**API Calls:**
- `PortalService/RotateKey`

**Step 1: Select Mode**

| Mode | Visual | Description |
|------|--------|-------------|
| ROTATION_IMMEDIATE | Lightning bolt | New key active now, old valid 24h |
| ROTATION_STAGED | Calendar | Both keys active for grace period |
| ROTATION_DUAL_SIGN | Two keys | New signs, old validates (strongest) |

Recommended: ROTATION_STAGED for production

**Step 2: Configure Grace Period (if STAGED)**
- Slider: 24h - 168h (1 week)
- Show: "Old key will stop working on [date]"

**Step 3: Confirmation**
- Show current key fingerprint
- Require typing "ROTATE" to confirm
- Checkbox: "I understand in-flight requests may need retry"

**Step 4: Result**
- Show new key fingerprint
- Copy button for new public key
- Timeline visualization of grace period
- Reminder to update deployments

**Error Handling:**
- `REGISTRY_ERROR_NO_ACTIVE_KEY` → "Generate a key first"
- `REGISTRY_ERROR_KEY_PENDING` → "Activate current key before rotating"

---

### 9. Key Management - Escrow Flow

**Purpose:** Create backup recovery capability for compliance.

**API Calls:**
- `PortalService/RequestKeyEscrow`
- `PortalService/ListKeyEscrows`
- `PortalService/RequestKeyRecovery`

**Escrow Creation:**

| Field | Options | Description |
|-------|---------|-------------|
| Escrow Type | STEWARD, ATTORNEY, DUAL_CUSTODY | Who holds the backup |
| Key to Escrow | Active keys dropdown | Which key to backup |

**Escrow List:**

| Column | Field |
|--------|-------|
| Key | key_id |
| Type | escrow_type |
| Custodian | custodian |
| Created | created_at |
| Status | status |

**Recovery Request Flow:**
1. Select escrowed key
2. Provide reason (required)
3. Sign request
4. Show: "Recovery pending steward approval. Request expires in 24 hours."

**UX Notes:**
- DUAL_CUSTODY shows: "Two stewards must approve recovery"
- Show compliance notice: "Required for HIPAA/SOX compliance"
- Link to documentation on recovery SLA

---

### 10. Audit Log Viewer

**Purpose:** View and search audit history.

**API Calls:**
- `PortalService/GetAuditLog`
- `PortalService/ExportAuditLog`

**Filters:**

| Filter | Type | Options |
|--------|------|---------|
| Date Range | Date picker | Last 24h, 7d, 30d, 90d, Custom |
| Action Type | Multi-select | KEY_GENERATED, USER_LOGIN, etc. |
| Actor | Search | User name/email |
| Target | Search | Entity ID |

**Action Types:**
```
Organization Events
├── AUDIT_ORG_CREATED
├── AUDIT_ORG_UPDATED
└── AUDIT_ORG_DELETED

User Events
├── AUDIT_USER_CREATED
├── AUDIT_USER_UPDATED
├── AUDIT_USER_LOGIN
└── AUDIT_USER_LOGOUT

Key Events
├── AUDIT_KEY_GENERATED
├── AUDIT_KEY_ACTIVATED
├── AUDIT_KEY_ROTATED
├── AUDIT_KEY_REVOKED
└── AUDIT_KEY_ESCROWED
```

**List View:**

| Column | Field |
|--------|-------|
| Timestamp | timestamp |
| Actor | actor_user_id (resolve to name) |
| Action | action (human-readable) |
| Target | target_type + target_id |
| IP | actor_ip_address |

**Detail Panel (on row click):**
- Full metadata map
- Description
- Signature verification status

**Export Options:**
| Format | Use Case |
|--------|----------|
| JSON | API integration |
| CSV | Spreadsheets |
| JSONL | Streaming/BigQuery |
| SPLUNK_HEC | SIEM integration |

---

### 11. Compliance Reports

**Purpose:** Generate compliance documentation.

**API Calls:**
- `PortalService/GenerateComplianceReport`

**Report Generator Form:**

| Field | Type | Options |
|-------|------|---------|
| Framework | Select | SOC2, HIPAA, GDPR, ISO27001, PCI_DSS |
| Period | Date range | Custom or "Last quarter" |
| Sections | Multi-select | key_management, access_control, audit |

**Report Display:**

Structure the report with collapsible sections:

```
SOC2 Compliance Report
├── Period: 2024-10-01 to 2025-01-01
├── Generated: 2025-01-26
│
├── Key Management Summary
│   ├── Keys Generated: 3
│   ├── Keys Rotated: 2
│   ├── Keys Revoked: 0
│   ├── Oldest Active Key: 45 days
│   └── Rotation Policy: COMPLIANT ✓
│
├── Access Control Summary
│   ├── Total Users: 12
│   ├── Admin Users: 2
│   ├── MFA Enabled: 10/12 (83%)
│   └── Failed Login Attempts: 5
│
├── Audit Summary
│   ├── Total Events: 1,234
│   ├── Audit Trail: CONTINUOUS ✓
│   └── Coverage: 100%
│
└── Attestation
    └── [Download signed PDF]
```

**Visual Indicators:**
- COMPLIANT: Green checkmark
- NON_COMPLIANT: Red X with remediation link
- PARTIAL: Yellow warning with details

---

### 12. Webhooks Configuration

**Purpose:** Configure event notifications.

**API Calls:**
- `RegistryAdminService/ListWebhooks`
- `RegistryAdminService/RegisterWebhook`
- `RegistryAdminService/DeleteWebhook`

**Webhook List:**

| Column | Field |
|--------|-------|
| URL | url (masked after domain) |
| Events | subscribed_events (badge count) |
| Status | active |
| Last Triggered | last_triggered_at |
| Failures | consecutive_failures |

**Create/Edit Form:**

| Field | Type | Validation |
|-------|------|------------|
| URL | URL input | HTTPS required |
| Events | Checkbox group | At least one |
| Active | Toggle | Default on |

**Event Categories:**
```
Agent Events
├── agent.registered
├── agent.deprecated
└── agent.revoked

Partner Events
├── partner.suspended
├── partner.revoked
└── partner.license_renewed

Key Events
├── key.generated
├── key.rotated
├── key.revoked
└── key.escrowed

System Events
├── emergency.shutdown
└── emergency.cleared
```

**Webhook Detail:**
- Show last 10 deliveries with status
- Retry failed deliveries button
- Test webhook button (sends test event)
- Signing secret (reveal on click, copy button)

---

## Admin Screens

### 13. Admin: Partner Management

**Purpose:** Manage licensed partners.

**API Calls:**
- `RegistryService/LookupPartner`
- `RegistryAdminService/RegisterPartner`
- `RegistryAdminService/ListExpiringLicenses`
- `RegistryAdminService/GetPartnerActivity`

**Partner List View:**

| Column | Field | Sortable |
|--------|-------|----------|
| Partner ID | partner_id | Yes |
| Organization | organization_name | Yes |
| License Type | license_type | Yes |
| Status | status | Yes |
| Expires | expires_at | Yes |
| Days Remaining | calculated | Yes |

**Partner Detail:**
- Organization info
- License details (type, issued, expires)
- Granted capabilities (tag list)
- Denied capabilities (tag list, if any)
- Activity summary (from GetPartnerActivity)
- Associated agents (if applicable)

**License Expiration Dashboard:**

| Section | Content |
|---------|---------|
| Expiring < 7 days | CRITICAL list |
| Expiring < 30 days | WARNING list |
| Expiring < 90 days | INFO list |
| Expired | OVERDUE list |

**Quick Actions:**
- Send renewal reminder email
- Extend license (admin override)
- Suspend partner

---

### 14. Admin: Agent Registry

**Purpose:** Manage registered agent builds.

**API Calls:**
- `RegistryService/LookupAgent`
- `RegistryService/BatchLookupAgents`
- `RegistryAdminService/RegisterAgent`
- `RegistryAdminService/BatchRegisterAgents`
- `RegistryAdminService/RegisterBuildAttestation`
- `RegistryService/GetBuildAttestation`

**Agent List View:**

| Column | Field |
|--------|-------|
| Hash (truncated) | agent_hash |
| Type | agent_type |
| Version | version (major.minor.patch) |
| Status | status |
| Registered | registered_at |
| Attestation | has_attestation icon |

**Agent Registration Form:**

| Field | Type | Required |
|-------|------|----------|
| Agent Hash | Hex input (64 chars) | Yes |
| Agent Type | Select | Yes |
| Version | Major.Minor.Patch | Yes |
| Base Capabilities | Multi-select/tags | Yes |
| Max Autonomy Tier | Select (A0-A4) | Yes |

**Batch Registration:**
- JSON file upload
- Preview table before submission
- Progress indicator during batch
- Results summary (success/failure counts)

**Build Attestation View:**

| Field | Description |
|-------|-------------|
| Build Type | GitHub Actions, GitLab CI, etc. |
| Source Repo | Git repository URL |
| Commit Hash | Git commit |
| Builder ID | CI system identifier |
| SLSA Level | Provenance level (0-4) |
| Signature | Attestation signature |

---

### 15. Admin: Incident Response Console

**Purpose:** Rapid response to security incidents (Admin only).

**API Calls:**
- `RegistryAdminService/MassRevoke`
- `RegistryAdminService/SetEmergencyShutdown`
- `RegistryAdminService/ClearEmergencyShutdown`
- `RegistryService/GetEmergencyStatus`
- `RegistryService/GetRevocationList`

**Emergency Status Banner:**
When `is_locked = true`, show prominent banner across all admin screens:
- Red background
- "EMERGENCY SHUTDOWN ACTIVE"
- Lock reason
- Time remaining (or "Manual unlock required")
- "Clear Emergency" button

**Mass Revocation Tool:**

| Selection Method | Input |
|------------------|-------|
| By Agent Hashes | Textarea (newline-separated) |
| By Partner IDs | Textarea (newline-separated) |
| By Version Pattern | Text input (e.g., "2.1.*") |
| By Agent Type | Dropdown |
| By Region | Multi-select ISO codes |
| By Registration Date | Date picker (before) |

**Workflow:**
1. Enter selection criteria
2. Click "Preview" (calls with `is_dry_run = true`)
3. Show affected count breakdown
4. Require typing incident ID to confirm
5. Execute revocation
6. Show result with audit log entry ID

**Emergency Shutdown:**

| Field | Type | Options |
|-------|------|---------|
| Severity | Select | LOW, MEDIUM, HIGH, CRITICAL |
| Reason | Textarea | Required |
| Duration | Select | 1h, 4h, 24h, 72h, Manual unlock |
| Allowed Operations | Multi-select | Empty = read-only |

**UX Notes:**
- Require two-factor confirmation for CRITICAL severity
- Show "This will affect X active deployments"
- Log all incident response actions prominently

**Revocation List View:**
- Searchable list of all revocations
- Filter by type (agent/partner/license)
- Filter by reason code
- Filter by severity
- Export capability

---

### 16. Admin: Registry Health Dashboard

**Purpose:** Monitor registry infrastructure.

**API Calls:**
- `RegistryService/HealthCheck` (with `include_diagnostics = true`)
- `RegistryService/GetMetrics`
- `RegistryAdminService/GetActiveSigningKey`
- `RegistryAdminService/ListSigningKeys`
- `RegistryAdminService/TestHSMConnection`

**Health Grid:**

| Component | HealthCheck Field | Visual |
|-----------|-------------------|--------|
| Overall Status | status | Large badge |
| Readiness | readiness | Badge |
| Database | database_healthy | Green/Red dot |
| Replication Lag | replication_lag_ms | Number with threshold coloring |
| HSM | components["hsm"] | Green/Red dot |

**Metrics Panels:**

| Panel | Data |
|-------|------|
| Query Rate | queries_total, queries_by_type chart |
| Latency | p50, p95, p99 as line chart |
| Error Rate | errors_total, errors_by_code pie chart |
| DB Pool | db_connections_active / db_connections_max |

**Signing Key Status:**
- Current active key fingerprint
- Dual signing status
- Key age and usage count
- "Rotate" button (leads to rotation flow)

**Signing Key Rotation (Admin):**
1. Select target storage (FILE, VAULT, HSM)
2. Preview new key generation
3. Confirm rotation
4. Monitor dual-signing period
5. Retire old key

**HSM Connection Test:**
- Select target HSM type
- Connection string input
- "Test Connection" button
- Show: connected, model, available slots

---

### 17. Admin: Offline Package Management

**Purpose:** Manage offline verification packages.

**API Calls:**
- `RegistryService/GetOfflinePackage`
- `RegistryService/GetOfflineDelta`

**Package Generation:**
- Generate new full package
- View package metadata (size, timestamp, expiration)
- Download package
- Verify signature

**Delta Management:**
- View available deltas
- Generate delta from timestamp
- Download delta package

---

## Error Handling Patterns

### Standard Error Toast

For non-critical errors, show toast notification:

```
┌─────────────────────────────────────┐
│ ⚠ Unable to load keys               │
│ REGISTRY_ERROR_DATABASE_ERROR       │
│                                     │
│ [Retry]  [Details]                  │
└─────────────────────────────────────┘
```

### Retry Guidance

Map `retry_status` to UX:

| Retry Status | UX Action |
|--------------|-----------|
| RETRY_NO | Show error, don't auto-retry |
| RETRY_IMMEDIATE | Auto-retry once, then show error |
| RETRY_BACKOFF | Show spinner, retry with backoff |
| RETRY_AFTER | Show countdown, retry after duration |

### Error Detail Modal

On "Details" click, show full error context:

```
Error Details
─────────────
Code: REGISTRY_ERROR_KEY_REVOKED
Message: Key abc123 was revoked on 2025-01-15

Metadata:
  key_id: abc123...
  revoked_by: user@example.com
  revocation_reason: Compromised

Cause:
  REGISTRY_ERROR_SECURITY_COMPROMISED
```

### Emergency Mode Handling

When emergency shutdown is active:
1. Show persistent banner on all pages
2. Disable write operations (gray out buttons)
3. Show clear explanation of what's blocked
4. Provide emergency contact information

---

## State Management

### Recommended Approach

Use request context for all API calls:

```javascript
const makeRequest = async (endpoint, params) => {
  const context = {
    request_id: uuid(),
    client_version: "portal-v2.0.0",
    user_agent: navigator.userAgent,
    request_timestamp: Date.now()
  };

  const response = await api.call(endpoint, { ...params, context });

  // Log correlation
  console.debug(`[${context.request_id}] ${endpoint} took ${response.context.processing_time_ms}ms`);

  return response;
};
```

### Caching Strategy

| Data Type | Cache Duration | Invalidation |
|-----------|----------------|--------------|
| Organization | 5 min | On update |
| User list | 2 min | On CRUD |
| Key list | 2 min | On CRUD |
| Audit log | No cache | Always fresh |
| Compliance report | 1 hour | Manual refresh |
| Health status | 30 sec | Auto-refresh |
| Emergency status | 10 sec | Auto-refresh |
| Partner list | 5 min | On CRUD |
| Agent list | 5 min | On CRUD |

### Real-time Updates

Consider WebSocket or polling for:
- Emergency status changes
- Key rotation notifications
- Audit log live tail (admin)
- Health status dashboard

---

## Accessibility Requirements

- All forms must have proper labels and ARIA attributes
- Status badges must have both color AND icon/text
- Keyboard navigation for all interactive elements
- Screen reader announcements for async operations
- Error messages must be linked to form fields
- Minimum contrast ratio 4.5:1
- Focus indicators visible on all interactive elements
- Skip links for main content areas
- Time-based operations must be pausable/extendable

### Color Blindness Considerations

Never rely on color alone. Combine with:
- Icons (checkmark, X, warning triangle)
- Text labels ("Active", "Revoked", "Pending")
- Patterns (solid, striped, dotted borders)

---

## API to Screen Mapping

| API Method | Primary Screen | Notes |
|------------|----------------|-------|
| HealthCheck | Health Status, Dashboard | Public |
| GetCapabilities | About/Settings | Version display |
| GetMetrics | Admin Health Dashboard | |
| LookupAgent | Agent Verification, Admin Agent List | |
| BatchLookupAgents | Admin Batch Operations | |
| LookupPartner | Dashboard, Admin Partner List | |
| VerifyDeployment | Debug/Test Tools | Admin only |
| GetRevocationList | Admin Incident Response | |
| GetPublicKeys | Key Details | |
| GetOfflinePackage | Admin Offline Management | |
| GetOfflineDelta | Admin Offline Management | |
| GetBuildAttestation | Admin Agent Details | |
| GetEmergencyStatus | All screens (banner) | |
| CreateOrganization | Onboarding | Admin flow |
| GetOrganization | Organization Settings | |
| UpdateOrganization | Organization Settings | |
| ListOrganizations | Admin Org List | Admin only |
| BatchCreateOrganizations | Admin Batch Import | |
| CreateOrgUser | User Management | |
| GetOrgUser | User Details | |
| GetOrgUserByEmail | User Search | |
| UpdateOrgUser | User Management | |
| ListOrgUsers | User Management | |
| BatchCreateOrgUsers | User Bulk Import | |
| GenerateKeyPair | Key Management | |
| ListKeys | Key Management | |
| ActivateKey | Key Management | |
| RotateKey | Key Rotation Flow | |
| RevokeKey | Key Management | |
| RequestKeyEscrow | Key Escrow | |
| RequestKeyRecovery | Key Escrow | |
| ListKeyEscrows | Key Escrow | |
| RequestSignature | Debug/API Tools | |
| GetAuditLog | Audit Log Viewer | |
| ExportAuditLog | Audit Log Viewer | |
| GenerateComplianceReport | Compliance Reports | |
| RegisterAgent | Admin Agent Registration | |
| BatchRegisterAgents | Admin Batch Operations | |
| RegisterPartner | Admin Partner Registration | |
| RevokeEntity | Admin Incident Response | |
| MassRevoke | Admin Incident Response | |
| SetEmergencyShutdown | Admin Incident Response | |
| ClearEmergencyShutdown | Admin Incident Response | |
| RotateSigningKey | Admin Signing Keys | |
| GetActiveSigningKey | Admin Health Dashboard | |
| ListSigningKeys | Admin Signing Keys | |
| TestHSMConnection | Admin HSM Management | |
| RegisterBuildAttestation | Admin Agent Registration | |
| RegisterWebhook | Webhooks Configuration | |
| ListWebhooks | Webhooks Configuration | |
| DeleteWebhook | Webhooks Configuration | |
| ListExpiringLicenses | Admin License Dashboard | |
| GetPartnerActivity | Admin Partner Details | |
| CleanupTestRecords | Admin Settings | Dev only |

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-01-26 | Initial specification |
| 2.0.0 | 2026-01-26 | Comprehensive update with all v1.1.0 endpoints |
