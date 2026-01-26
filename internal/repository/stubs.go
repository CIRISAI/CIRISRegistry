// Package repository provides data access layer for CIRISRegistry.
// This file contains stub implementations - replace with real implementations.
package repository

import (
	"context"
	"database/sql"
	"errors"
)

var ErrNotImplemented = errors.New("not implemented")

// ============================================================================
// Agent Repository Stub
// ============================================================================

type agentRepo struct {
	db *sql.DB
}

func (r *agentRepo) Get(ctx context.Context, hash []byte) (*Agent, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *agentRepo) Create(ctx context.Context, agent *Agent) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *agentRepo) UpdateStatus(ctx context.Context, hash []byte, status AgentStatus, reason string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *agentRepo) List(ctx context.Context, opts ListAgentsOptions) ([]Agent, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

// ============================================================================
// Partner Repository Stub
// ============================================================================

type partnerRepo struct {
	db *sql.DB
}

func (r *partnerRepo) Get(ctx context.Context, partnerID string) (*Partner, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *partnerRepo) Create(ctx context.Context, partner *Partner) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *partnerRepo) Update(ctx context.Context, partner *Partner) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *partnerRepo) UpdateStatus(ctx context.Context, partnerID string, status PartnerStatus, reason string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *partnerRepo) List(ctx context.Context, opts ListPartnersOptions) ([]Partner, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

// ============================================================================
// Organization Repository Stub
// ============================================================================

type orgRepo struct {
	db *sql.DB
}

func (r *orgRepo) Get(ctx context.Context, orgID string) (*Organization, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *orgRepo) GetByPartnerID(ctx context.Context, partnerID string) (*Organization, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *orgRepo) Create(ctx context.Context, org *Organization) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *orgRepo) Update(ctx context.Context, org *Organization) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *orgRepo) Deactivate(ctx context.Context, orgID string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *orgRepo) List(ctx context.Context, opts ListOrgsOptions) ([]Organization, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

// ============================================================================
// User Repository Stub
// ============================================================================

type userRepo struct {
	db *sql.DB
}

func (r *userRepo) Get(ctx context.Context, userID string) (*OrgUser, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *userRepo) GetByEmail(ctx context.Context, email string) (*OrgUser, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *userRepo) GetByOAuth(ctx context.Context, provider, subject string) (*OrgUser, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *userRepo) Create(ctx context.Context, user *OrgUser) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *userRepo) Update(ctx context.Context, user *OrgUser) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *userRepo) Deactivate(ctx context.Context, userID string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *userRepo) List(ctx context.Context, orgID string, opts ListUsersOptions) ([]OrgUser, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

func (r *userRepo) RecordLogin(ctx context.Context, userID string) error {
	// TODO: Implement
	return ErrNotImplemented
}

// ============================================================================
// Key Repository Stub
// ============================================================================

type keyRepo struct {
	db *sql.DB
}

func (r *keyRepo) Get(ctx context.Context, keyID string) (*PartnerKey, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *keyRepo) GetActiveForOrg(ctx context.Context, orgID string) (*PartnerKey, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *keyRepo) Create(ctx context.Context, key *PartnerKey) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *keyRepo) Activate(ctx context.Context, keyID string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *keyRepo) Rotate(ctx context.Context, oldKeyID, newKeyID string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *keyRepo) Revoke(ctx context.Context, keyID string, reason string, revokedBy string) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *keyRepo) List(ctx context.Context, orgID string, opts ListKeysOptions) ([]PartnerKey, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

// ============================================================================
// Revocation Repository Stub
// ============================================================================

type revocationRepo struct {
	db *sql.DB
}

func (r *revocationRepo) Get(ctx context.Context, targetType RevocationType, targetID string) (*Revocation, error) {
	// TODO: Implement
	return nil, ErrNotImplemented
}

func (r *revocationRepo) Create(ctx context.Context, rev *Revocation) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *revocationRepo) List(ctx context.Context, sinceVersion int64) ([]Revocation, int64, error) {
	// TODO: Implement
	return nil, 0, ErrNotImplemented
}

// ============================================================================
// Audit Repository Stub
// ============================================================================

type auditRepo struct {
	db *sql.DB
}

func (r *auditRepo) Log(ctx context.Context, entry *AuditEntry) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *auditRepo) List(ctx context.Context, opts ListAuditOptions) ([]AuditEntry, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}

// ============================================================================
// Signing Log Repository Stub
// ============================================================================

type signingLogRepo struct {
	db *sql.DB
}

func (r *signingLogRepo) Log(ctx context.Context, entry *SigningLogEntry) error {
	// TODO: Implement
	return ErrNotImplemented
}

func (r *signingLogRepo) List(ctx context.Context, opts ListSigningLogOptions) ([]SigningLogEntry, string, error) {
	// TODO: Implement
	return nil, "", ErrNotImplemented
}
