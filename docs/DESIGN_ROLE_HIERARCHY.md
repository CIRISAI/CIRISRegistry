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

### 4a. User Model (Multi-Org)

```protobuf
// User identity (org-independent)
message User {
  string user_id = 1;
  string email = 2;
  string name = 3;
  string oauth_provider = 10;
  string oauth_subject = 11;
  bool active = 20;
  int64 created_at = 21;
  int64 updated_at = 22;
  int64 last_login_at = 23;
  string created_at_iso = 24;
  string updated_at_iso = 25;
  string last_login_at_iso = 26;
  bool mfa_enabled = 30;
  string mfa_method = 31;

  // Memberships this user has (populated on fetch)
  repeated OrgMembership memberships = 40;
}

// User's membership in a specific org
message OrgMembership {
  string org_id = 1;
  string org_name = 2;              // Denormalized for convenience
  OrgType org_type = 3;             // Denormalized
  OrgRole role = 4;
  string invited_by = 5;
  int64 created_at = 6;
  string created_at_iso = 7;
}
```

**Key Difference from OrgUser:**
- `OrgUser` was tied to a single org (had `org_id` as core field)
- `User` is identity-only; memberships are a list
- A user can be `ORG_ADMIN` in Org A and `ORG_VIEWER` in Org B

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

-- System users table (global admins, separate from org membership)
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

-- Refactored: Users table (identity only, no org affiliation)
-- Migration: Extract user identity from org_users
CREATE TABLE users (
  user_id TEXT PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  oauth_provider TEXT,
  oauth_subject TEXT,
  active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_login_at TIMESTAMPTZ,
  mfa_enabled BOOLEAN NOT NULL DEFAULT false,
  mfa_method TEXT
);

-- Junction table: User ↔ Organization with role
-- Replaces org_users (which had org_id baked in)
CREATE TABLE user_org_memberships (
  user_id TEXT NOT NULL REFERENCES users(user_id),
  org_id TEXT NOT NULL REFERENCES organizations(org_id),
  role INTEGER NOT NULL,  -- OrgRole enum
  invited_by TEXT REFERENCES users(user_id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, org_id)
);

-- Index for "list users in org" queries
CREATE INDEX idx_user_org_memberships_org ON user_org_memberships(org_id);
```

**Migration Strategy:**
1. Create new `users` and `user_org_memberships` tables
2. Migrate data from `org_users`:
   - Insert into `users` (dedupe by email if needed)
   - Insert into `user_org_memberships` with existing role
3. Update application code to use new schema
4. Drop `org_users` table

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

## Design Decisions

1. **LICENSEE orgs do NOT inherit capabilities from parent PARTNER**
   - LICENSEE may be a vertical (e.g., medical clinic) where parent is a horizontal/SI
   - Each LICENSEE has their own PartnerRecord with explicit capability grants
   - Parent relationship is organizational/billing, not capability inheritance

2. **Users can belong to multiple orgs with different roles**
   - Remove org_id from OrgUser, use junction table instead
   - User can be ADMIN in one org, VIEWER in another
   - System users are still separate (global access)

3. **Community → Partner upgrade supported**
   - Requires Steward action (SYSTEM_ADMIN)
   - Creates PartnerRecord with license
   - Changes org_type to PARTNER
   - Existing users/data preserved

4. **Rate limiting by org type**
   - INTERNAL: No limits
   - PARTNER: Standard limits
   - LICENSEE: Own limits (not inherited from parent)
   - COMMUNITY: Stricter limits

---

## Summary

| Entity | Changes |
|--------|---------|
| **Organization** | +org_type, +parent_org_id |
| **OrgRole** | Unchanged (org-scoped) |
| **User** | New table (replaces OrgUser identity) |
| **OrgMembership** | Junction table: user ↔ org with role |
| **SystemUser** | New table/message (global admins) |
| **SystemRole** | New enum (ADMIN, AUDITOR, WISE_AUTHORITY) |
| **PortalService** | +CreateSystemUser, +ListSystemUsers, +CreateLicenseeOrganization, +AddUserToOrg, +RemoveUserFromOrg |

### Key Design Decisions

1. **LICENSEE does NOT inherit parent capabilities** - Verticals have their own license
2. **Multi-org membership** - User can have different roles in different orgs
3. **Community → Partner upgrade** - Supported via SYSTEM_ADMIN action

### Breaking Changes

- `OrgUser` replaced by `User` + `OrgMembership`
- Portal must update to:
  - Create user first, then add to org
  - Or use atomic `CreateUserWithMembership` (new RPC)

---

*Awaiting approval before implementation.*
