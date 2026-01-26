// Package service implements the gRPC service handlers for CIRISRegistry.
package service

import (
	"context"
	"log/slog"

	"github.com/cirisai/cirisregistry/internal/repository"
)

// AdminService implements the RegistryAdminService gRPC interface.
// This service handles administrative operations (requires authentication).
type AdminService struct {
	repos  *repository.Repositories
	logger *slog.Logger
	// TODO: Add signer for record signatures
}

// NewAdminService creates a new AdminService.
func NewAdminService(repos *repository.Repositories, logger *slog.Logger) *AdminService {
	return &AdminService{
		repos:  repos,
		logger: logger,
	}
}

// RegisterAgent adds a new agent build to the registry.
func (s *AdminService) RegisterAgent(ctx context.Context, agent *repository.Agent) error {
	s.logger.Info("registering agent",
		"agent_type", agent.Type,
		"version", agent.Version,
	)

	// TODO: Verify admin signature
	// TODO: Sign agent record

	if err := s.repos.Agents.Create(ctx, agent); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		Action:      "AGENT_REGISTERED",
		TargetType:  "agent",
		TargetID:    string(agent.Hash),
		Description: "Agent build registered",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// RegisterPartner adds a new licensed partner to the registry.
func (s *AdminService) RegisterPartner(ctx context.Context, partner *repository.Partner) error {
	s.logger.Info("registering partner",
		"partner_id", partner.PartnerID,
		"organization", partner.OrganizationName,
		"license_type", partner.LicenseType,
	)

	// TODO: Verify steward signature
	// TODO: Sign partner record

	if err := s.repos.Partners.Create(ctx, partner); err != nil {
		return err
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		Action:      "PARTNER_REGISTERED",
		TargetType:  "partner",
		TargetID:    partner.PartnerID,
		Description: "Partner license registered",
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}

// RevokeEntity adds an entity to the revocation list.
func (s *AdminService) RevokeEntity(ctx context.Context, targetType repository.RevocationType, targetID string, reason string, detail string) error {
	s.logger.Info("revoking entity",
		"target_type", targetType,
		"target_id", targetID,
		"reason", reason,
	)

	// TODO: Verify authority signature
	// TODO: Sign revocation entry

	rev := &repository.Revocation{
		TargetType:   targetType,
		TargetID:     targetID,
		ReasonCode:   reason,
		ReasonDetail: detail,
	}

	if err := s.repos.Revocations.Create(ctx, rev); err != nil {
		return err
	}

	// Update status on target entity
	switch targetType {
	case repository.RevocationTypeAgentHash:
		// Agent hash is hex-encoded in targetID
		// TODO: Decode and update agent status
	case repository.RevocationTypePartnerID:
		if err := s.repos.Partners.UpdateStatus(ctx, targetID, repository.PartnerStatusRevoked, detail); err != nil {
			s.logger.Warn("failed to update partner status", "error", err)
		}
	}

	// Audit log
	if err := s.repos.Audit.Log(ctx, &repository.AuditEntry{
		Action:      "ENTITY_REVOKED",
		TargetType:  string(targetType),
		TargetID:    targetID,
		Description: detail,
	}); err != nil {
		s.logger.Warn("failed to log audit entry", "error", err)
	}

	return nil
}
