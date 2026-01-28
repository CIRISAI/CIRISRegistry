# CIRISPortal UI/UX Updates for Role Hierarchy

**Version:** 1.2.0
**Date:** 2026-01-28
**Status:** Implementation Guide for Portal Team

---

## Summary

This document describes the UI/UX changes required in CIRISPortal to support the new role hierarchy and multi-organization membership features implemented in CIRISRegistry v1.2.0.

### Key Changes

1. **Organization Types** - Organizations now have types (INTERNAL, PARTNER, LICENSEE, COMMUNITY)
2. **Organization Hierarchy** - PARTNER orgs can create LICENSEE child organizations
3. **Multi-Org Membership** - Users can belong to multiple organizations with different roles
4. **System Users** - Global administrators separate from org users

---

## 1. Organizations Page Updates

### 1.1 Organization List View

**Current State:** Flat list of organizations

**New Features:**

| Element | Description |
|---------|-------------|
| **Org Type Badge** | Display organization type as a colored badge |
| **Parent Indicator** | Show parent org name for LICENSEE orgs |
| **Hierarchy Icon** | Show expand/collapse for orgs with children |
| **Type Filter** | Add filter dropdown: All / INTERNAL / PARTNER / LICENSEE / COMMUNITY |

**Badge Colors:**
```
INTERNAL   → Purple (#7C3AED)
PARTNER    → Blue (#2563EB)
LICENSEE   → Teal (#0D9488)
COMMUNITY  → Gray (#6B7280)
```

**Example Row:**
```
┌─────────────────────────────────────────────────────────────────────┐
│ ▼ Acme Healthcare [PARTNER]                    Created: Jan 15, 2026│
│   └─ Acme Clinic Boston [LICENSEE]             Parent: Acme Health  │
│   └─ Acme Clinic NYC [LICENSEE]                Parent: Acme Health  │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Create Organization Dialog

**Current Fields:**
- Name, Legal Name, Primary Email, etc.

**New Fields:**

| Field | Type | Visibility | Notes |
|-------|------|------------|-------|
| **Organization Type** | Dropdown | Always | Default: COMMUNITY |
| **Parent Organization** | Searchable Dropdown | If type = LICENSEE | Required for LICENSEE |

**Validation Rules:**
- INTERNAL: Only @ciris.ai users can create
- PARTNER: Only SYSTEM_ADMIN or INTERNAL org admins can create
- LICENSEE: Must select a PARTNER parent; only PARTNER org admins can create
- COMMUNITY: Anyone can create (default)

**Conditional UI:**
```javascript
// When type changes
if (orgType === 'LICENSEE') {
  showField('parentOrganization');
  setFieldRequired('parentOrganization', true);
} else {
  hideField('parentOrganization');
}
```

### 1.3 Organization Detail Page

**New Sections:**

#### Hierarchy Panel (for PARTNER/LICENSEE types)
```
┌─────────────────────────────────────────────┐
│ Organization Hierarchy                       │
├─────────────────────────────────────────────┤
│ Parent: Acme Healthcare [PARTNER] → View    │
│                                             │
│ Children (3):                               │
│   • Acme Clinic Boston [LICENSEE]           │
│   • Acme Clinic NYC [LICENSEE]              │
│   • Acme Clinic Chicago [LICENSEE]          │
│                                             │
│ [+ Create Licensee Organization]            │
└─────────────────────────────────────────────┘
```

#### Type Information Banner
```
┌─────────────────────────────────────────────┐
│ ⓘ PARTNER Organization                       │
│ This organization can create and manage     │
│ licensee organizations.                      │
└─────────────────────────────────────────────┘
```

---

## 2. User Management Updates

### 2.1 New User Model

**Key Change:** Users are now **org-independent identities** with **memberships** in organizations.

**Before (OrgUser):**
```json
{
  "user_id": "...",
  "org_id": "org-123",  // User bound to single org
  "email": "...",
  "role": "ORG_ADMIN"
}
```

**After (User + Memberships):**
```json
{
  "user_id": "...",
  "email": "...",
  "name": "...",
  "memberships": [
    { "org_id": "org-123", "org_name": "Acme", "role": "ORG_ADMIN" },
    { "org_id": "org-456", "org_name": "Beta Corp", "role": "ORG_VIEWER" }
  ]
}
```

### 2.2 Organization Users Tab

**Current State:** Simple user list

**New Features:**

| Feature | Description |
|---------|-------------|
| **Role Indicator** | Show role with icon/badge |
| **Multi-Org Indicator** | Show badge if user belongs to other orgs |
| **Add to Org** | Add existing user to this org (vs. create new) |

**User Row Example:**
```
┌─────────────────────────────────────────────────────────────────────┐
│ Jane Doe                                                            │
│ jane@acme.com                     [ORG_ADMIN] [+2 other orgs] ...  │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.3 Create/Add User Dialog

**Two Modes:**

1. **Create New User** - Creates user identity and adds to org
2. **Add Existing User** - Search for existing user by email, add to org

**Dialog Flow:**
```
┌──────────────────────────────────────────────────────────────┐
│ Add User to Organization                                      │
├──────────────────────────────────────────────────────────────┤
│ ○ Create new user                                            │
│ ● Add existing user                                          │
│                                                              │
│ Email: [jane@example.com        ] [Search]                   │
│                                                              │
│ ┌────────────────────────────────────────────────────────┐  │
│ │ Found: Jane Doe (jane@example.com)                     │  │
│ │ Current memberships: Acme Corp (ADMIN), Beta (VIEWER)  │  │
│ └────────────────────────────────────────────────────────┘  │
│                                                              │
│ Role in this org: [ORG_VIEWER ▼]                            │
│                                                              │
│                              [Cancel] [Add User]             │
└──────────────────────────────────────────────────────────────┘
```

### 2.4 User Detail Page

**New Sections:**

#### Memberships Panel
```
┌─────────────────────────────────────────────────────────────────┐
│ Organization Memberships (3)                                     │
├─────────────────────────────────────────────────────────────────┤
│ Organization          │ Type      │ Role        │ Actions       │
│───────────────────────┼───────────┼─────────────┼───────────────│
│ Acme Healthcare       │ PARTNER   │ ORG_ADMIN   │ [Edit] [Remove]│
│ Beta Corp             │ COMMUNITY │ ORG_VIEWER  │ [Edit] [Remove]│
│ Acme Clinic Boston    │ LICENSEE  │ ORG_OPERATOR│ [Edit] [Remove]│
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. System Users (New Screen)

### 3.1 Navigation

Add to sidebar (visible only to SYSTEM_ADMIN users):

```
├── Organizations
├── Users
├── System Users        ← NEW
├── Keys
└── Audit Log
```

### 3.2 System Users List

```
┌─────────────────────────────────────────────────────────────────────┐
│ System Users                                    [+ Add System User] │
├─────────────────────────────────────────────────────────────────────┤
│ Name            │ Email              │ Role            │ Status     │
│─────────────────┼────────────────────┼─────────────────┼────────────│
│ Admin User      │ admin@ciris.ai     │ SYSTEM_ADMIN    │ ● Active   │
│ Auditor         │ auditor@ciris.ai   │ SYSTEM_AUDITOR  │ ● Active   │
│ Wise Member 1   │ wise1@example.com  │ WISE_AUTHORITY  │ ● Active   │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.3 System Roles

| Role | Description | Email Requirement |
|------|-------------|-------------------|
| SYSTEM_ADMIN | Full system access | Must be @ciris.ai |
| SYSTEM_AUDITOR | Read-only across all orgs | Any email |
| WISE_AUTHORITY | Governance access (9 max) | Any email |

### 3.4 Create System User Dialog

```
┌──────────────────────────────────────────────────────────────┐
│ Create System User                                            │
├──────────────────────────────────────────────────────────────┤
│ Name:  [                    ]                                │
│ Email: [                    ]                                │
│ Role:  [SYSTEM_ADMIN ▼     ]                                │
│                                                              │
│ ⚠️ SYSTEM_ADMIN requires @ciris.ai email                     │
│                                                              │
│                              [Cancel] [Create]               │
└──────────────────────────────────────────────────────────────┘
```

---

## 4. Role Clarification UI

### 4.1 Role Badges

Display roles consistently with icons:

```
ORG_ADMIN      → 👑 Admin (Blue)
ORG_KEY_MANAGER→ 🔑 Key Manager (Yellow)
ORG_OPERATOR   → ⚙️ Operator (Green)
ORG_VIEWER     → 👁️ Viewer (Gray)

SYSTEM_ADMIN   → 🛡️ System Admin (Purple)
SYSTEM_AUDITOR → 📋 Auditor (Teal)
WISE_AUTHORITY → ⚖️ Wise Authority (Gold)
```

### 4.2 Role Permissions Matrix (Help Modal)

Add "What can each role do?" help button:

```
┌───────────────────────────────────────────────────────────────────┐
│ Organization Role Permissions                              [×]    │
├───────────────────────────────────────────────────────────────────┤
│ Permission          │ ADMIN │ KEY_MANAGER │ OPERATOR │ VIEWER    │
│─────────────────────┼───────┼─────────────┼──────────┼───────────│
│ View org details    │   ✓   │      ✓      │    ✓     │     ✓     │
│ Manage users        │   ✓   │      ✗      │    ✗     │     ✗     │
│ Manage keys         │   ✓   │      ✓      │    ✗     │     ✗     │
│ Request signatures  │   ✓   │      ✓      │    ✓     │     ✗     │
│ View audit log      │   ✓   │      ✓      │    ✓     │     ✓     │
│ Export audit log    │   ✓   │      ✗      │    ✗     │     ✗     │
│ Create licensees*   │   ✓   │      ✗      │    ✗     │     ✗     │
├───────────────────────────────────────────────────────────────────┤
│ * Only for PARTNER type organizations                             │
└───────────────────────────────────────────────────────────────────┘
```

---

## 5. API Integration Guide

### 5.1 New Endpoints

| Endpoint | Purpose | Use Case |
|----------|---------|----------|
| `CreateUser` | Create user identity | User creation flow |
| `CreateUserWithMembership` | Create user + add to org | Quick add flow |
| `GetUser` | Get user with memberships | User detail page |
| `AddUserToOrg` | Add existing user to org | Add to org flow |
| `RemoveUserFromOrg` | Remove user from org | Remove from org |
| `UpdateUserOrgRole` | Change user's role in org | Edit role |
| `ListOrgMembers` | List org users | Organization users tab |
| `CreateSystemUser` | Create system user | System users page |
| `ListSystemUsers` | List system users | System users page |
| `ListChildOrganizations` | Get org children | Hierarchy view |
| `CreateLicenseeOrganization` | Create child org | Create licensee flow |
| `GetOrganizationHierarchy` | Get full hierarchy | Org detail page |
| `UpgradeToPartner` | Upgrade COMMUNITY→PARTNER | Admin action |

### 5.2 Response Changes

**Organization response now includes:**
```typescript
interface Organization {
  // existing fields...
  org_type: OrgType;       // NEW: INTERNAL, PARTNER, LICENSEE, COMMUNITY
  parent_org_id: string;   // NEW: Parent org ID (if LICENSEE)
}
```

**User response (new model):**
```typescript
interface User {
  user_id: string;
  email: string;
  name: string;
  active: boolean;
  created_at_iso: string;   // ISO 8601 timestamp
  updated_at_iso: string;   // ISO 8601 timestamp
  memberships: OrgMembership[];  // NEW: All org memberships
}

interface OrgMembership {
  org_id: string;
  org_name: string;
  org_type: OrgType;
  role: OrgRole;
  invited_by: string;
  created_at_iso: string;
}
```

### 5.3 Backward Compatibility

The legacy `OrgUser` endpoints still work during transition:
- `CreateOrgUser` - Creates user bound to single org (legacy)
- `GetOrgUser` - Returns OrgUser format (legacy)
- `ListOrgUsers` - Returns list of OrgUser (legacy)

**Migration Path:**
1. Update UI to use new `User` + membership endpoints
2. Replace `OrgUser` references with `User` + `OrgMembership`
3. Legacy endpoints will be deprecated in v1.3.0

---

## 6. Implementation Checklist

### Phase 1: Organization Types
- [ ] Add org_type badge to organization list
- [ ] Add org_type field to create organization dialog
- [ ] Show parent/children in org detail page
- [ ] Add type filter to organization list

### Phase 2: Multi-Org Users
- [ ] Update user list to show membership count
- [ ] Add "Add existing user" option to user dialog
- [ ] Create user memberships panel in user detail
- [ ] Add edit/remove membership actions

### Phase 3: System Users
- [ ] Create System Users navigation item
- [ ] Implement System Users list page
- [ ] Implement Create System User dialog
- [ ] Add email validation for SYSTEM_ADMIN role

### Phase 4: Polish
- [ ] Add role permissions help modal
- [ ] Update role badges with icons/colors
- [ ] Add hierarchy visualization (tree view)
- [ ] Update all "created_at" displays to use ISO timestamps

---

## 7. Design Assets Needed

| Asset | Description |
|-------|-------------|
| Org type badges | INTERNAL, PARTNER, LICENSEE, COMMUNITY badges |
| Role badges | All org and system role badges with icons |
| Hierarchy icons | Expand/collapse, parent link, child link |
| Empty states | No children, no memberships, etc. |

---

*Questions? Contact registry@ciris.ai*
