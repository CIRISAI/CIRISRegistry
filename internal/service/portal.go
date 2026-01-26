// Package service implements the gRPC service handlers for CIRISRegistry.
package service

import (
	"context"
	"errors"
	"log/slog"

	"github.com/cirisai/cirisregistry/internal/repository"
)

var (
	ErrUnauthorized      = errors.New("unauthorized")
	ErrOrgNotFound       = errors.New("organization not found")
	ErrUserNotFound      = errors.New("user not found")
	ErrKeyNotFound       = errors.New("key not found")
	ErrKeyAlreadyActive  = errors.New("organization already has an active key")
	ErrNotImplemented    = errors.New("not implemented")
)

// PortalService implements the PortalService gRPC interface.
// This service handles CIRISPortal operations (organization, user, key management).
type PortalService struct {
	repos  *repository.Repositories
	logger *slog.Logger
	// TODO: Add KeyStore interface for custodied key operations
	// TODO: Add OAuth verifier
}

// NewPortalService creates a new PortalService.
func NewPortalService(repos *repository.Repositories, logger *slog.Logger) *PortalService {
	return &PortalService{
		repos:  repos,
		logger: logger,
	}
}

// ============================================================================
// Organization Management
// ============================================================================

// CreateOrganization creates a new organization.
func (s *PortalService) CreateOrganization(ctx context.Context, org *repository.Organization, actorUserID string) error {
	s.logger.Info("creating organization",
		"name", org.Name,
		"actor", actorUserID,
	)

	// TODO: Verify wise authority authorization

	org.CreatedBy = actorUserID
	if err := s.repos.Organizations.Create(ctx, org); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		Action:      "ORG_CREATED",
		TargetType:  "organization",
		TargetID:    org.OrgID,
		Description: "Organization created: " + org.Name,
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// GetOrganization retrieves an organization by ID.
func (s *PortalService) GetOrganization(ctx context.Context, orgID string) (*repository.Organization, error) {
	return s.repos.Organizations.Get(ctx, orgID)
}

// UpdateOrganization modifies an existing organization.
func (s *PortalService) UpdateOrganization(ctx context.Context, org *repository.Organization, actorUserID string) error {
	s.logger.Info("updating organization",
		"org_id", org.OrgID,
		"actor", actorUserID,
	)

	// TODO: Verify actor has ORG_ADMIN role

	if err := s.repos.Organizations.Update(ctx, org); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  org.OrgID,
		Action:      "ORG_UPDATED",
		TargetType:  "organization",
		TargetID:    org.OrgID,
		Description: "Organization updated",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// ListOrganizations returns organizations with pagination.
func (s *PortalService) ListOrganizations(ctx context.Context, opts repository.ListOrgsOptions) ([]repository.Organization, string, int, error) {
	orgs, nextToken, err := s.repos.Organizations.List(ctx, opts)
	if err != nil {
		return nil, "", 0, err
	}
	// TODO: Get total count
	return orgs, nextToken, len(orgs), nil
}

// ============================================================================
// User Management
// ============================================================================

// CreateOrgUser creates a new user in an organization.
func (s *PortalService) CreateOrgUser(ctx context.Context, user *repository.OrgUser, actorUserID string) error {
	s.logger.Info("creating user",
		"email", user.Email,
		"org_id", user.OrgID,
		"actor", actorUserID,
	)

	// TODO: Verify actor has ORG_ADMIN role for this org

	user.InvitedBy = actorUserID
	if err := s.repos.Users.Create(ctx, user); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  user.OrgID,
		Action:      "USER_CREATED",
		TargetType:  "user",
		TargetID:    user.UserID,
		Description: "User created: " + user.Email,
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// GetOrgUser retrieves a user by ID.
func (s *PortalService) GetOrgUser(ctx context.Context, userID string) (*repository.OrgUser, error) {
	return s.repos.Users.Get(ctx, userID)
}

// GetOrgUserByEmail retrieves a user by email.
func (s *PortalService) GetOrgUserByEmail(ctx context.Context, email string) (*repository.OrgUser, error) {
	return s.repos.Users.GetByEmail(ctx, email)
}

// UpdateOrgUser modifies an existing user.
func (s *PortalService) UpdateOrgUser(ctx context.Context, user *repository.OrgUser, actorUserID string) error {
	s.logger.Info("updating user",
		"user_id", user.UserID,
		"actor", actorUserID,
	)

	// TODO: Verify actor has ORG_ADMIN role or is the user themselves

	if err := s.repos.Users.Update(ctx, user); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  user.OrgID,
		Action:      "USER_UPDATED",
		TargetType:  "user",
		TargetID:    user.UserID,
		Description: "User updated",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// ListOrgUsers returns users in an organization with pagination.
func (s *PortalService) ListOrgUsers(ctx context.Context, orgID string, opts repository.ListUsersOptions) ([]repository.OrgUser, string, int, error) {
	users, nextToken, err := s.repos.Users.List(ctx, orgID, opts)
	if err != nil {
		return nil, "", 0, err
	}
	return users, nextToken, len(users), nil
}

// ============================================================================
// Key Management
// ============================================================================

// GenerateKeyPair generates a new hybrid key pair for an organization.
func (s *PortalService) GenerateKeyPair(ctx context.Context, orgID string, actorUserID string, activateImmediately bool) (*repository.PartnerKey, error) {
	s.logger.Info("generating key pair",
		"org_id", orgID,
		"actor", actorUserID,
		"activate", activateImmediately,
	)

	// TODO: Verify actor has ORG_ADMIN or ORG_KEY_MANAGER role

	// Check if org already has active key (if activating immediately)
	if activateImmediately {
		existing, err := s.repos.Keys.GetActiveForOrg(ctx, orgID)
		if err != nil && !errors.Is(err, repository.ErrNotImplemented) {
			return nil, err
		}
		if existing != nil {
			return nil, ErrKeyAlreadyActive
		}
	}

	// TODO: Generate Ed25519 key pair
	// TODO: Generate ML-DSA-65 key pair
	// TODO: Store encrypted private keys in KeyStore (CloudflareKV/HSM)
	// TODO: Compute fingerprints

	key := &repository.PartnerKey{
		OrgID:        orgID,
		CustodyModel: "CUSTODIED",
		Status:       repository.KeyStatusPending,
		CreatedBy:    actorUserID,
		// Ed25519PublicKey: ...,
		// MLDSA65PublicKey: ...,
		// KVKeyRef: ...,
	}

	if activateImmediately {
		key.Status = repository.KeyStatusActive
	}

	if err := s.repos.Keys.Create(ctx, key); err != nil {
		return nil, err
	}

	// Audit log
	action := "KEY_GENERATED"
	if activateImmediately {
		action = "KEY_ACTIVATED"
	}
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  orgID,
		Action:      action,
		TargetType:  "key",
		TargetID:    key.KeyID,
		Description: "Key pair generated",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return key, nil
}

// ListKeys returns keys for an organization with pagination.
func (s *PortalService) ListKeys(ctx context.Context, orgID string, opts repository.ListKeysOptions) ([]repository.PartnerKey, string, int, error) {
	keys, nextToken, err := s.repos.Keys.List(ctx, orgID, opts)
	if err != nil {
		return nil, "", 0, err
	}
	return keys, nextToken, len(keys), nil
}

// ActivateKey activates a pending key.
func (s *PortalService) ActivateKey(ctx context.Context, orgID string, keyID string, actorUserID string) error {
	s.logger.Info("activating key",
		"org_id", orgID,
		"key_id", keyID,
		"actor", actorUserID,
	)

	// TODO: Verify actor has ORG_ADMIN or ORG_KEY_MANAGER role
	// TODO: Verify key belongs to org and is PENDING

	if err := s.repos.Keys.Activate(ctx, keyID); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  orgID,
		Action:      "KEY_ACTIVATED",
		TargetType:  "key",
		TargetID:    keyID,
		Description: "Key activated",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// RotateKey generates a new key and marks the old one as rotated.
func (s *PortalService) RotateKey(ctx context.Context, orgID string, actorUserID string, reason string) (*repository.PartnerKey, *repository.PartnerKey, error) {
	s.logger.Info("rotating key",
		"org_id", orgID,
		"actor", actorUserID,
		"reason", reason,
	)

	// TODO: Verify actor has ORG_ADMIN or ORG_KEY_MANAGER role

	// Get current active key
	oldKey, err := s.repos.Keys.GetActiveForOrg(ctx, orgID)
	if err != nil {
		return nil, nil, err
	}
	if oldKey == nil {
		return nil, nil, ErrKeyNotFound
	}

	// Generate new key
	newKey, err := s.GenerateKeyPair(ctx, orgID, actorUserID, false)
	if err != nil {
		return nil, nil, err
	}

	// Rotate (marks old as ROTATED, new as ACTIVE)
	if err := s.repos.Keys.Rotate(ctx, oldKey.KeyID, newKey.KeyID); err != nil {
		return nil, nil, err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  orgID,
		Action:      "KEY_ROTATED",
		TargetType:  "key",
		TargetID:    oldKey.KeyID,
		Description: "Key rotated: " + reason,
		Metadata: map[string]string{
			"old_key_id": oldKey.KeyID,
			"new_key_id": newKey.KeyID,
			"reason":     reason,
		},
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return oldKey, newKey, nil
}

// RevokeKey marks a key as revoked.
func (s *PortalService) RevokeKey(ctx context.Context, orgID string, keyID string, reason string, actorUserID string) error {
	s.logger.Info("revoking key",
		"org_id", orgID,
		"key_id", keyID,
		"reason", reason,
		"actor", actorUserID,
	)

	// TODO: Verify actor has ORG_ADMIN or ORG_KEY_MANAGER role
	// TODO: Verify key belongs to org

	if err := s.repos.Keys.Revoke(ctx, keyID, reason, actorUserID); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		ActorUserID: actorUserID,
		ActorOrgID:  orgID,
		Action:      "KEY_REVOKED",
		TargetType:  "key",
		TargetID:    keyID,
		Description: "Key revoked: " + reason,
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// ============================================================================
// Signing (Custodied Keys)
// ============================================================================

// RequestSignature signs data using an organization's custodied key.
func (s *PortalService) RequestSignature(ctx context.Context, orgID string, keyID string, data []byte, purpose string, actorUserID string) (*SignatureResult, error) {
	s.logger.Info("signing data",
		"org_id", orgID,
		"key_id", keyID,
		"purpose", purpose,
		"actor", actorUserID,
	)

	// TODO: Verify actor belongs to org
	// TODO: Get key from KeyStore
	// TODO: Sign with Ed25519
	// TODO: Sign with ML-DSA-65
	// TODO: Record in signing log

	return nil, ErrNotImplemented
}

// SignatureResult holds the result of a signing operation.
type SignatureResult struct {
	ClassicalSignature   []byte
	PostQuantumSignature []byte
	KeyID                string
	SignedAt             int64
}

// ============================================================================
// Audit Log
// ============================================================================

// GetAuditLog retrieves audit log entries.
func (s *PortalService) GetAuditLog(ctx context.Context, opts repository.ListAuditOptions) ([]repository.AuditEntry, string, int, error) {
	entries, nextToken, err := s.repos.Audit.List(ctx, opts)
	if err != nil {
		return nil, "", 0, err
	}
	return entries, nextToken, len(entries), nil
}
