# CIRISRegistry Role Hierarchy and Nested Organizations Design

**Version:** Draft 1.0
**Date:** 2026-01-28
**Status:** Proposal

---

## Background

The current CIRISRegistry schema has flat organizations with org-scoped roles only. This doesn't map to the CIRIS ecosystem licensing model which has three distinct layers:

1. **CIRIS Steward** (Internal) - Maintains open stack, coordinates network
2. **Licensed Delegated Authority** (Partners) - Steward-backed, can certify others
3. **Community** (Anyone) - Unlicensed, self-aware of limitations

Additionally, licensed partners need to create **Licensee** organizations beneath them (e.g., a healthcare partner onboards individual clinics).

---

## Proposed Design

### 1. Organization Types

```protobuf
enum OrgType {
  ORG_TYPE_UNSPECIFIED = 0;
  ORG_INTERNAL = 1;         // CIRIS L3C (@ciris.ai only)
  ORG_PARTNER = 2;          // Licensed Delegated Authority
  ORG_LICENSEE = 3;         // Under a partner (vertical deployment)
  ORG_COMMUNITY = 4;        // Unlicensed (cannot claim official status)
}
```

| Type | Can Create Children | Can Issue Official Scores | Steward Backing |
|------|---------------------|---------------------------|-----------------|
| INTERNAL | PARTNER, LICENSEE, COMMUNITY | Yes | Is the Steward |
| PARTNER | LICENSEE | Yes | Yes |
| LICENSEE | No | No (uses parent's) | Indirect (via partner) |
| COMMUNITY | No | No | No |

### 2. Organization Hierarchy

```protobuf
message Organization {
  string org_id = 1;
  string name = 2;
  string legal_name = 3;

  // NEW: Organization type and hierarchy
  OrgType org_type = 5;
  string parent_org_id = 6;           // NULL for INTERNAL/top-level PARTNER

  // Existing fields...
  string partner_id = 10;             // Links to PartnerRecord (license)
  // ...
}
```

**Hierarchy Rules:**
- `INTERNAL` orgs have no parent (root)
- `PARTNER` orgs may have no parent (direct with Steward) or parent = INTERNAL
- `LICENSEE` orgs must have parent = PARTNER
- `COMMUNITY` orgs may have no parent

### 3. System-Level Roles (Global)

```protobuf
enum SystemRole {
  SYSTEM_ROLE_UNSPECIFIED = 0;
  SYSTEM_ADMIN = 1;           // Full system access (@ciris.ai only)
  SYSTEM_AUDITOR = 2;         // Read-only across all orgs (for compliance)
  WISE_AUTHORITY = 3;         // Governance access (9 members)
}

message SystemUser {
  string user_id = 1;
  string email = 2;               // Must be @ciris.ai for SYSTEM_ADMIN
  string name = 3;
  SystemRole role = 4;
  bool active = 5;
  int64 created_at = 6;
  int64 updated_at = 7;
  string created_at_iso = 8;
  string updated_at_iso = 9;
}
```

**Constraints:**
- `SYSTEM_ADMIN` requires email ending in `@ciris.ai`
- `WISE_AUTHORITY` has 9 members max, staggered 3-year terms
- System users are distinct from org users (can exist without org membership)

### 4. Organization Roles (Unchanged, Clarified)

```protobuf
enum OrgRole {
  ORG_ROLE_UNSPECIFIED = 0;
  ORG_ADMIN = 1;                  // Full organization management
  ORG_KEY_MANAGER = 2;           // Key operations only
  ORG_OPERATOR = 3;              // View + limited operations
  ORG_VIEWER = 4;                // Read-only access
}
```

**Org Role Permissions:**

| Permission | ADMIN | KEY_MANAGER | OPERATOR | VIEWER |
|------------|-------|-------------|----------|--------|
| View org details | ✅ | ✅ | ✅ | ✅ |
| Manage users | ✅ | ❌ | ❌ | ❌ |
| Manage keys | ✅ | ✅ | ❌ | ❌ |
| Request signatures | ✅ | ✅ | ✅ | ❌ |
| View audit log | ✅ | ✅ | ✅ | ✅ |
| Export audit log | ✅ | ❌ | ❌ | ❌ |
| Create licensee orgs | ✅* | ❌ | ❌ | ❌ |

*Only for PARTNER orgs

### 5. License Tiers (Clarified Naming)

The existing `LicenseType` in `PartnerRecord` maps to capability grants:

| License Type | Org Type | Capabilities |
|--------------|----------|--------------|
| COMMUNITY | OrgType.COMMUNITY | Basic, no official status |
| COMMUNITY_PLUS | OrgType.COMMUNITY | Extended wellness/info |
| PROFESSIONAL_MEDICAL | OrgType.PARTNER | domain:medical:* |
| PROFESSIONAL_LEGAL | OrgType.PARTNER | domain:legal:* |
| PROFESSIONAL_FINANCIAL | OrgType.PARTNER | domain:financial:* |
| PROFESSIONAL_FULL | OrgType.PARTNER | All professional |

### 6. Database Schema Changes

```sql
-- Add org_type column
ALTER TABLE organizations
ADD COLUMN org_type INTEGER NOT NULL DEFAULT 4;  -- Default COMMUNITY

-- Add parent_org_id column
ALTER TABLE organizations
ADD COLUMN parent_org_id TEXT REFERENCES organizations(org_id);

-- Index for hierarchy queries
CREATE INDEX idx_organizations_parent ON organizations(parent_org_id);

-- System users table (separate from org_users)
CREATE TABLE system_users (
  user_id TEXT PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  role INTEGER NOT NULL,  -- SystemRole enum
  active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT email_domain_check CHECK (
    role != 1 OR email LIKE '%@ciris.ai'  -- SYSTEM_ADMIN must be @ciris.ai
  )
);
```

### 7. API Changes

#### New RPC Methods

```protobuf
service PortalService {
  // Existing...

  // NEW: System user management (SYSTEM_ADMIN only)
  rpc CreateSystemUser(CreateSystemUserRequest) returns (CreateSystemUserResponse);
  rpc ListSystemUsers(ListSystemUsersRequest) returns (ListSystemUsersResponse);
  rpc UpdateSystemUser(UpdateSystemUserRequest) returns (AdminResponse);

  // NEW: Organization hierarchy
  rpc ListChildOrganizations(ListChildOrganizationsRequest) returns (ListOrganizationsResponse);
  rpc CreateLicenseeOrganization(CreateLicenseeOrgRequest) returns (CreateOrganizationResponse);
}

service RegistryService {
  // NEW: Public hierarchy query
  rpc GetOrganizationHierarchy(GetOrgHierarchyRequest) returns (GetOrgHierarchyResponse);
}
```

#### Modified Requests

```protobuf
message CreateOrganizationRequest {
  Organization organization = 1;
  OrgUser initial_admin = 3;
  OrgType org_type = 4;           // NEW: Required for non-COMMUNITY
  string parent_org_id = 5;       // NEW: Required for LICENSEE
  RequestContext context = 10;
}
```

### 8. Authorization Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        Request Arrives                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│            1. Check System User (if system operation)           │
│                                                                 │
│   Is user in system_users table with appropriate SystemRole?    │
│   • SYSTEM_ADMIN: Full access                                   │
│   • SYSTEM_AUDITOR: Read-only everywhere                        │
│   • WISE_AUTHORITY: Governance operations only                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│            2. Check Org User (if org operation)                 │
│                                                                 │
│   Is user in org_users for target organization?                 │
│   Check OrgRole permissions for requested operation.            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│            3. Check Org Type Constraints                        │
│                                                                 │
│   • PARTNER creating LICENSEE: Allowed                          │
│   • LICENSEE creating child: Denied                             │
│   • COMMUNITY creating child: Denied                            │
│   • Any org accessing parent's resources: Check inheritance     │
└─────────────────────────────────────────────────────────────────┘
```

### 9. UI Implications (CIRISPortal)

| Screen | Changes Needed |
|--------|----------------|
| **Organizations List** | Add org_type badge, show hierarchy tree |
| **Create Organization** | Add org_type selector, parent org picker (for licensees) |
| **Organization Detail** | Show child orgs (if PARTNER), show parent (if LICENSEE) |
| **System Users** | New screen (SYSTEM_ADMIN only) |
| **User Create** | Clarify: "This user will be scoped to [Org Name]" |

### 10. Migration Path

1. **Phase 1**: Add columns with defaults (backward compatible)
   - All existing orgs default to `OrgType.COMMUNITY`
   - No existing orgs have parents

2. **Phase 2**: Migrate known organizations
   - Set CIRIS L3C org to `OrgType.INTERNAL`
   - Set existing partners to `OrgType.PARTNER`

3. **Phase 3**: Enable hierarchy features
   - Portal UI shows hierarchy
   - Licensee creation enabled

---

## Open Questions

1. **Should LICENSEE orgs inherit capabilities from parent PARTNER?**
   - Option A: Yes, automatic inheritance
   - Option B: No, explicit grants required
   - Recommendation: Option A with ability to restrict

2. **Can a user belong to multiple orgs?**
   - Current: No (org_id is required on OrgUser)
   - Proposal: Allow multi-org membership via junction table
   - Recommendation: Keep single-org for simplicity, system users handle cross-org access

3. **How do COMMUNITY orgs get upgraded to PARTNER?**
   - Requires Steward action (SYSTEM_ADMIN)
   - Creates PartnerRecord with license
   - Changes org_type to PARTNER

4. **Rate limiting by org type?**
   - INTERNAL: No limits
   - PARTNER: Standard limits
   - LICENSEE: Inherit parent limits
   - COMMUNITY: Stricter limits

---

## Summary

| Entity | Changes |
|--------|---------|
| **Organization** | +org_type, +parent_org_id |
| **OrgRole** | Unchanged (org-scoped) |
| **SystemUser** | New table/message |
| **SystemRole** | New enum (ADMIN, AUDITOR, WISE_AUTHORITY) |
| **PortalService** | +CreateSystemUser, +ListSystemUsers, +CreateLicenseeOrganization |

This design maintains backward compatibility while enabling the hierarchical model needed for the CIRIS ecosystem licensing structure.

---

*Awaiting review before implementation.*
