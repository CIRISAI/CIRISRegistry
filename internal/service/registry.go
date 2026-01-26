// Package service implements the gRPC service handlers for CIRISRegistry.
package service

import (
	"context"
	"log/slog"

	"github.com/cirisai/cirisregistry/internal/repository"
)

// RegistryService implements the public RegistryService gRPC interface.
// This service handles read-only agent/partner lookups.
type RegistryService struct {
	repos  *repository.Repositories
	logger *slog.Logger
	// TODO: Add signer for response signatures
}

// NewRegistryService creates a new RegistryService.
func NewRegistryService(repos *repository.Repositories, logger *slog.Logger) *RegistryService {
	return &RegistryService{
		repos:  repos,
		logger: logger,
	}
}

// LookupAgent retrieves an agent record by hash.
func (s *RegistryService) LookupAgent(ctx context.Context, hash []byte, nonce []byte) (*AgentLookupResult, error) {
	s.logger.Debug("looking up agent", "hash_len", len(hash))

	agent, err := s.repos.Agents.Get(ctx, hash)
	if err != nil {
		return nil, err
	}

	// TODO: Sign response
	// TODO: Generate Merkle proof

	return &AgentLookupResult{
		Agent: agent,
		Found: agent != nil,
	}, nil
}

// LookupPartner retrieves a partner record by ID.
func (s *RegistryService) LookupPartner(ctx context.Context, partnerID string, nonce []byte) (*PartnerLookupResult, error) {
	s.logger.Debug("looking up partner", "partner_id", partnerID)

	partner, err := s.repos.Partners.Get(ctx, partnerID)
	if err != nil {
		return nil, err
	}

	// TODO: Sign response
	// TODO: Generate Merkle proof

	return &PartnerLookupResult{
		Partner: partner,
		Found:   partner != nil,
	}, nil
}

// VerifyDeployment performs combined agent + partner verification.
func (s *RegistryService) VerifyDeployment(ctx context.Context, agentHash []byte, partnerID string, nonce []byte) (*DeploymentVerificationResult, error) {
	s.logger.Debug("verifying deployment",
		"agent_hash_len", len(agentHash),
		"partner_id", partnerID,
	)

	agent, err := s.repos.Agents.Get(ctx, agentHash)
	if err != nil {
		return nil, err
	}

	partner, err := s.repos.Partners.Get(ctx, partnerID)
	if err != nil {
		return nil, err
	}

	// TODO: Compute effective capabilities
	// TODO: Sign response

	return &DeploymentVerificationResult{
		Agent:      agent,
		Partner:    partner,
		AgentFound: agent != nil,
		PartnerFound: partner != nil,
		// EffectiveCapabilities: computed,
		// MandatoryDisclosure: disclosure,
	}, nil
}

// GetRevocationList returns the current revocation list.
func (s *RegistryService) GetRevocationList(ctx context.Context, sinceVersion int64) (*RevocationListResult, error) {
	s.logger.Debug("getting revocation list", "since_version", sinceVersion)

	revocations, version, err := s.repos.Revocations.List(ctx, sinceVersion)
	if err != nil {
		return nil, err
	}

	return &RevocationListResult{
		Revocations: revocations,
		Version:     version,
		IsDelta:     sinceVersion > 0,
	}, nil
}

// GetPublicKeys retrieves public keys for an organization.
func (s *RegistryService) GetPublicKeys(ctx context.Context, orgID string, keyID string) (*PublicKeysResult, error) {
	s.logger.Debug("getting public keys", "org_id", orgID, "key_id", keyID)

	var key *repository.PartnerKey
	var err error

	if keyID != "" {
		key, err = s.repos.Keys.Get(ctx, keyID)
	} else {
		key, err = s.repos.Keys.GetActiveForOrg(ctx, orgID)
	}
	if err != nil {
		return nil, err
	}

	return &PublicKeysResult{
		Key:   key,
		Found: key != nil,
	}, nil
}

// ============================================================================
// Result Types
// ============================================================================

type AgentLookupResult struct {
	Agent *repository.Agent
	Found bool
	// TODO: Signature, MerkleProof
}

type PartnerLookupResult struct {
	Partner *repository.Partner
	Found   bool
	// TODO: Signature, MerkleProof
}

type DeploymentVerificationResult struct {
	Agent                 *repository.Agent
	Partner               *repository.Partner
	AgentFound            bool
	PartnerFound          bool
	EffectiveCapabilities []string
	EffectiveAutonomy     string
	MandatoryDisclosure   string
	// TODO: Signature
}

type RevocationListResult struct {
	Revocations []repository.Revocation
	Version     int64
	IsDelta     bool
	// TODO: Signature
}

type PublicKeysResult struct {
	Key   *repository.PartnerKey
	Found bool
}
