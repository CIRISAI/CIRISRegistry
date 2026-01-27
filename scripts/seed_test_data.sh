#!/bin/bash
# CIRISRegistry Test Data Seeding Script
# Usage: ./seed_test_data.sh [grpc_endpoint]

set -e

ENDPOINT="${1:-localhost:50052}"
GRPCURL="${GRPCURL:-grpcurl}"

echo "=== CIRISRegistry Test Data Seeder ==="
echo "Endpoint: $ENDPOINT"
echo ""

# Check grpcurl is available
if ! command -v $GRPCURL &> /dev/null; then
    echo "Error: grpcurl not found. Install from https://github.com/fullstorydev/grpcurl"
    exit 1
fi

# Test connectivity
echo "1. Testing connectivity..."
HEALTH=$($GRPCURL -plaintext -d '{}' $ENDPOINT ciris.registry.v1.RegistryService/HealthCheck 2>&1)
if echo "$HEALTH" | grep -q "HEALTH_SERVING"; then
    echo "   OK - Server is healthy"
else
    echo "   FAILED - Server not responding"
    echo "$HEALTH"
    exit 1
fi

# Create primary test organization
echo ""
echo "2. Creating primary test organization..."
ORG_RESPONSE=$($GRPCURL -plaintext -d '{
  "context": {"request_id": "seed-org-1"},
  "organization": {
    "name": "QA Primary Organization",
    "legal_name": "QA Primary Organization Inc.",
    "tax_id": "99-1234567",
    "primary_email": "admin@qa-primary.test",
    "billing_email": "billing@qa-primary.test",
    "technical_contact_email": "tech@qa-primary.test",
    "oauth_provider": "google",
    "oauth_domain": "qa-primary.test"
  }
}' $ENDPOINT ciris.registry.v1.PortalService/CreateOrganization 2>&1)

ORG_ID=$(echo "$ORG_RESPONSE" | grep -o '"message": "Organization created with ID: [^"]*"' | sed 's/.*ID: //' | tr -d '"')
if [ -z "$ORG_ID" ]; then
    echo "   FAILED - Could not create organization"
    echo "$ORG_RESPONSE"
    exit 1
fi
echo "   OK - Created org: $ORG_ID"

# Create secondary test organization
echo ""
echo "3. Creating secondary test organization..."
ORG2_RESPONSE=$($GRPCURL -plaintext -d '{
  "context": {"request_id": "seed-org-2"},
  "organization": {
    "name": "QA Secondary Organization",
    "legal_name": "QA Secondary Organization LLC",
    "primary_email": "admin@qa-secondary.test",
    "oauth_provider": "okta",
    "oauth_domain": "qa-secondary.test"
  }
}' $ENDPOINT ciris.registry.v1.PortalService/CreateOrganization 2>&1)

ORG2_ID=$(echo "$ORG2_RESPONSE" | grep -o '"message": "Organization created with ID: [^"]*"' | sed 's/.*ID: //' | tr -d '"')
echo "   OK - Created org: $ORG2_ID"

# Create admin user for primary org
echo ""
echo "4. Creating admin user for primary org..."
USER_RESPONSE=$($GRPCURL -plaintext -d "{
  \"context\": {\"request_id\": \"seed-user-1\"},
  \"user\": {
    \"org_id\": \"$ORG_ID\",
    \"email\": \"admin@qa-primary.test\",
    \"name\": \"QA Admin User\",
    \"role\": 1,
    \"oauth_provider\": \"google\",
    \"oauth_subject\": \"google-oauth2|admin123\"
  }
}" $ENDPOINT ciris.registry.v1.PortalService/CreateOrgUser 2>&1)

USER_ID=$(echo "$USER_RESPONSE" | grep -o '"message": "User created with ID: [^"]*"' | sed 's/.*ID: //' | tr -d '"')
if [ -z "$USER_ID" ]; then
    echo "   FAILED - Could not create user"
    echo "$USER_RESPONSE"
    exit 1
fi
echo "   OK - Created user: $USER_ID"

# Create regular user for primary org
echo ""
echo "5. Creating regular user for primary org..."
USER2_RESPONSE=$($GRPCURL -plaintext -d "{
  \"context\": {\"request_id\": \"seed-user-2\"},
  \"user\": {
    \"org_id\": \"$ORG_ID\",
    \"email\": \"user@qa-primary.test\",
    \"name\": \"QA Regular User\",
    \"role\": 4,
    \"oauth_provider\": \"google\",
    \"oauth_subject\": \"google-oauth2|user456\"
  }
}" $ENDPOINT ciris.registry.v1.PortalService/CreateOrgUser 2>&1)

USER2_ID=$(echo "$USER2_RESPONSE" | grep -o '"message": "User created with ID: [^"]*"' | sed 's/.*ID: //' | tr -d '"')
echo "   OK - Created user: $USER2_ID"

# Generate and activate key for primary org
echo ""
echo "6. Generating key pair for primary org..."
KEY_RESPONSE=$($GRPCURL -plaintext -d "{
  \"context\": {\"request_id\": \"seed-key-1\"},
  \"org_id\": \"$ORG_ID\",
  \"requester_user_id\": \"$USER_ID\",
  \"activate_immediately\": true
}" $ENDPOINT ciris.registry.v1.PortalService/GenerateKeyPair 2>&1)

KEY_ID=$(echo "$KEY_RESPONSE" | grep -o '"keyId": "[^"]*"' | head -1 | sed 's/"keyId": "//' | tr -d '"')
if [ -z "$KEY_ID" ]; then
    echo "   FAILED - Could not generate key"
    echo "$KEY_RESPONSE"
    exit 1
fi
echo "   OK - Generated key: $KEY_ID"

# Test signature with the new key
echo ""
echo "7. Testing signature operation..."
SIG_RESPONSE=$($GRPCURL -plaintext -d "{
  \"context\": {\"request_id\": \"seed-sig-1\"},
  \"sign_request\": {
    \"org_id\": \"$ORG_ID\",
    \"data\": \"SGVsbG8gV29ybGQh\",
    \"purpose\": \"Test signature\"
  }
}" $ENDPOINT ciris.registry.v1.PortalService/RequestSignature 2>&1)

if echo "$SIG_RESPONSE" | grep -q '"success": true'; then
    echo "   OK - Signature created successfully"
else
    echo "   FAILED - Signature operation failed"
    echo "$SIG_RESPONSE"
fi

# Summary
echo ""
echo "=== Seed Data Summary ==="
echo ""
echo "Primary Organization:"
echo "  org_id:  $ORG_ID"
echo "  name:    QA Primary Organization"
echo "  domain:  qa-primary.test"
echo ""
echo "Secondary Organization:"
echo "  org_id:  $ORG2_ID"
echo "  name:    QA Secondary Organization"
echo "  domain:  qa-secondary.test"
echo ""
echo "Users (Primary Org):"
echo "  admin:   $USER_ID (admin@qa-primary.test)"
echo "  user:    $USER2_ID (user@qa-primary.test)"
echo ""
echo "Keys (Primary Org):"
echo "  key_id:  $KEY_ID (active)"
echo ""
echo "=== Seeding Complete ==="
echo ""
echo "You can now test with:"
echo "  grpcurl -plaintext -d '{\"org_id\": \"$ORG_ID\"}' $ENDPOINT ciris.registry.v1.PortalService/ListOrgUsers"
