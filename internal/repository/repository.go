// Package repository provides data access layer for CIRISRegistry.
package repository

import (
	"context"
	"database/sql"
	"time"
)

// AgentRepository handles agent record persistence.
type AgentRepository interface {
	// Get retrieves an agent by hash.
	Get(ctx context.Context, hash []byte) (*Agent, error)
	// Create stores a new agent record.
	Create(ctx context.Context, agent *Agent) error
	// UpdateStatus changes agent status.
	UpdateStatus(ctx context.Context, hash []byte, status AgentStatus, reason string) error
	// List returns agents with pagination.
	List(ctx context.Context, opts ListAgentsOptions) ([]Agent, string, error)
}

// PartnerRepository handles partner record persistence.
type PartnerRepository interface {
	// Get retrieves a partner by ID.
	Get(ctx context.Context, partnerID string) (*Partner, error)
	// Create stores a new partner record.
	Create(ctx context.Context, partner *Partner) error
	// Update modifies an existing partner.
	Update(ctx context.Context, partner *Partner) error
	// UpdateStatus changes partner status.
	UpdateStatus(ctx context.Context, partnerID string, status PartnerStatus, reason string) error
	// List returns partners with pagination.
	List(ctx context.Context, opts ListPartnersOptions) ([]Partner, string, error)
}

// OrganizationRepository handles organization persistence.
type OrganizationRepository interface {
	// Get retrieves an organization by ID.
	Get(ctx context.Context, orgID string) (*Organization, error)
	// GetByPartnerID retrieves organization by partner ID.
	GetByPartnerID(ctx context.Context, partnerID string) (*Organization, error)
	// Create stores a new organization.
	Create(ctx context.Context, org *Organization) error
	// Update modifies an existing organization.
	Update(ctx context.Context, org *Organization) error
	// Deactivate soft-deletes an organization.
	Deactivate(ctx context.Context, orgID string) error
	// List returns organizations with pagination.
	List(ctx context.Context, opts ListOrgsOptions) ([]Organization, string, error)
}

// UserRepository handles organization user persistence.
type UserRepository interface {
	// Get retrieves a user by ID.
	Get(ctx context.Context, userID string) (*OrgUser, error)
	// GetByEmail retrieves a user by email.
	GetByEmail(ctx context.Context, email string) (*OrgUser, error)
	// GetByOAuth retrieves a user by OAuth subject.
	GetByOAuth(ctx context.Context, provider, subject string) (*OrgUser, error)
	// Create stores a new user.
	Create(ctx context.Context, user *OrgUser) error
	// Update modifies an existing user.
	Update(ctx context.Context, user *OrgUser) error
	// Deactivate soft-deletes a user.
	Deactivate(ctx context.Context, userID string) error
	// List returns users for an organization.
	List(ctx context.Context, orgID string, opts ListUsersOptions) ([]OrgUser, string, error)
	// RecordLogin updates last login time.
	RecordLogin(ctx context.Context, userID string) error
}

// KeyRepository handles partner key persistence.
type KeyRepository interface {
	// Get retrieves a key by ID.
	Get(ctx context.Context, keyID string) (*PartnerKey, error)
	// GetActiveForOrg retrieves the active key for an organization.
	GetActiveForOrg(ctx context.Context, orgID string) (*PartnerKey, error)
	// Create stores a new key record.
	Create(ctx context.Context, key *PartnerKey) error
	// Activate sets a key as the active key for its org.
	Activate(ctx context.Context, keyID string) error
	// Rotate marks old key as rotated and activates new key.
	Rotate(ctx context.Context, oldKeyID, newKeyID string) error
	// Revoke marks a key as revoked.
	Revoke(ctx context.Context, keyID string, reason string, revokedBy string) error
	// List returns keys for an organization.
	List(ctx context.Context, orgID string, opts ListKeysOptions) ([]PartnerKey, string, error)
}

// RevocationRepository handles revocation list persistence.
type RevocationRepository interface {
	// Get retrieves a revocation by target.
	Get(ctx context.Context, targetType RevocationType, targetID string) (*Revocation, error)
	// Create stores a new revocation.
	Create(ctx context.Context, rev *Revocation) error
	// List returns all revocations, optionally since a version.
	List(ctx context.Context, sinceVersion int64) ([]Revocation, int64, error)
}

// AuditRepository handles audit log persistence.
type AuditRepository interface {
	// Log records an audit entry.
	Log(ctx context.Context, entry *AuditEntry) error
	// List returns audit entries with filters.
	List(ctx context.Context, opts ListAuditOptions) ([]AuditEntry, string, error)
}

// SigningLogRepository handles signing log persistence.
type SigningLogRepository interface {
	// Log records a signing operation.
	Log(ctx context.Context, entry *SigningLogEntry) error
	// List returns signing log entries.
	List(ctx context.Context, opts ListSigningLogOptions) ([]SigningLogEntry, string, error)
}

// Repositories aggregates all repository interfaces.
type Repositories struct {
	Agents        AgentRepository
	Partners      PartnerRepository
	Organizations OrganizationRepository
	Users         UserRepository
	Keys          KeyRepository
	Revocations   RevocationRepository
	Audit         AuditRepository
	SigningLog    SigningLogRepository
}

// NewRepositories creates repository implementations from a database connection.
func NewRepositories(db *sql.DB) *Repositories {
	return &Repositories{
		Agents:        &agentRepo{db: db},
		Partners:      &partnerRepo{db: db},
		Organizations: &orgRepo{db: db},
		Users:         &userRepo{db: db},
		Keys:          &keyRepo{db: db},
		Revocations:   &revocationRepo{db: db},
		Audit:         &auditRepo{db: db},
		SigningLog:    &signingLogRepo{db: db},
	}
}

// ============================================================================
// Domain Types (simplified - match proto definitions)
// ============================================================================

type AgentStatus string

const (
	AgentStatusActive     AgentStatus = "ACTIVE"
	AgentStatusDeprecated AgentStatus = "DEPRECATED"
	AgentStatusRevoked    AgentStatus = "REVOKED"
)

type PartnerStatus string

const (
	PartnerStatusActive    PartnerStatus = "ACTIVE"
	PartnerStatusSuspended PartnerStatus = "SUSPENDED"
	PartnerStatusRevoked   PartnerStatus = "REVOKED"
)

type KeyStatus string

const (
	KeyStatusPending KeyStatus = "PENDING"
	KeyStatusActive  KeyStatus = "ACTIVE"
	KeyStatusRotated KeyStatus = "ROTATED"
	KeyStatusRevoked KeyStatus = "REVOKED"
)

type RevocationType string

const (
	RevocationTypeAgentHash RevocationType = "AGENT_HASH"
	RevocationTypePartnerID RevocationType = "PARTNER_ID"
	RevocationTypeLicenseID RevocationType = "LICENSE_ID"
)

// Agent represents an agent build record.
type Agent struct {
	Hash             []byte
	Type             string
	Version          string
	BaseCapabilities []string
	MaxAutonomyTier  string
	BuildTimestamp   time.Time
	SourceRepo       string
	SourceCommit     string
	Status           AgentStatus
	RevocationReason string
	RegisteredAt     time.Time
	LastUpdated      time.Time
}

// Partner represents a licensed partner organization.
type Partner struct {
	PartnerID            string
	OrganizationName     string
	OrganizationID       string
	LicenseType          string
	LicenseID            string
	IssuedAt             time.Time
	ExpiresAt            time.Time
	CapabilitiesGranted  []string
	CapabilitiesDenied   []string
	MaxAutonomyTier      string
	RequiresSupervisor   bool
	GeographicRestrict   []string
	DeploymentLimit      int
	OfflineGraceHours    int
	TechnicalContact     string
	ComplianceContact    string
	Status               PartnerStatus
	SuspensionReason     string
	RevocationReason     string
	StatusChangedAt      time.Time
	CreatedAt            time.Time
	UpdatedAt            time.Time
}

// Organization represents a portal-managed organization.
type Organization struct {
	OrgID                  string
	Name                   string
	LegalName              string
	TaxID                  string
	PartnerID              string
	PrimaryEmail           string
	BillingEmail           string
	TechnicalContactEmail  string
	ComplianceContactEmail string
	OAuthProvider          string
	OAuthDomain            string
	Active                 bool
	CreatedAt              time.Time
	UpdatedAt              time.Time
	CreatedBy              string
	Metadata               map[string]string
}

// OrgUser represents a user within an organization.
type OrgUser struct {
	UserID        string
	OrgID         string
	Email         string
	Name          string
	OAuthProvider string
	OAuthSubject  string
	Role          string
	Active        bool
	CreatedAt     time.Time
	UpdatedAt     time.Time
	LastLoginAt   *time.Time
	InvitedBy     string
	MFAEnabled    bool
	MFAMethod     string
}

// PartnerKey represents a key pair record.
type PartnerKey struct {
	KeyID               string
	OrgID               string
	PartnerID           string
	Ed25519PublicKey    []byte
	MLDSA65PublicKey    []byte
	Ed25519Fingerprint  string
	MLDSA65Fingerprint  string
	CustodyModel        string
	KVKeyRef            string
	Status              KeyStatus
	RevocationReason    string
	CreatedAt           time.Time
	ActivatedAt         *time.Time
	RotatedAt           *time.Time
	RevokedAt           *time.Time
	CreatedBy           string
	RotatedBy           string
	RevokedBy           string
}

// Revocation represents a revocation entry.
type Revocation struct {
	ID           string
	TargetType   RevocationType
	TargetID     string
	RevokedAt    time.Time
	ReasonCode   string
	ReasonDetail string
	Severity     string
}

// AuditEntry represents an audit log entry.
type AuditEntry struct {
	EntryID        string
	Timestamp      time.Time
	ActorUserID    string
	ActorOrgID     string
	ActorIPAddress string
	ActorUserAgent string
	Action         string
	TargetType     string
	TargetID       string
	Description    string
	Metadata       map[string]string
}

// SigningLogEntry represents a signing operation log entry.
type SigningLogEntry struct {
	LogID           string
	OrgID           string
	KeyID           string
	RequesterUserID string
	DataHash        []byte
	Purpose         string
	SignedAt        time.Time
	IPAddress       string
	UserAgent       string
}

// ============================================================================
// List Options
// ============================================================================

type ListAgentsOptions struct {
	PageSize   int
	PageToken  string
	Status     *AgentStatus
	AgentType  string
}

type ListPartnersOptions struct {
	PageSize  int
	PageToken string
	Status    *PartnerStatus
}

type ListOrgsOptions struct {
	PageSize        int
	PageToken       string
	IncludeInactive bool
}

type ListUsersOptions struct {
	PageSize        int
	PageToken       string
	IncludeInactive bool
}

type ListKeysOptions struct {
	PageSize       int
	PageToken      string
	IncludeRevoked bool
}

type ListAuditOptions struct {
	OrgID       string
	StartTime   time.Time
	EndTime     time.Time
	ActionTypes []string
	PageSize    int
	PageToken   string
}

type ListSigningLogOptions struct {
	OrgID     string
	KeyID     string
	StartTime time.Time
	EndTime   time.Time
	PageSize  int
	PageToken string
}
