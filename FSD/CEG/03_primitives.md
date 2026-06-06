# CEG Primitives

## Overview

CEG (CIRIS Event Grammar) defines a set of primitives that form the foundation for expressing events and their relationships. These primitives are designed to be composable and extensible.

## Core Primitives

The following primitives are defined:

### 1. Actor
Represents an entity that can perform actions within the system.

### 2. Action
An operation or activity performed by an actor.

### 3. Asset
A resource that can be referenced, transferred, or modified.

### 4. Event
A temporal occurrence that can be recorded and referenced.

### 5. Relation
A connection between two or more primitives.

### 6. Envelope
A container for grouping related primitives.

### 7. Receipt
A record of completion or acknowledgment of an action.

## Primitive: Receipt (Settlement Linkage)

**NEW:** The `Receipt` primitive serves as an optional cohort-scoped attestation mechanism for commerce actions and their on-chain settlement linkage. This enables auditable traces of value transfers while maintaining privacy-by-default through cohort scoping.

### Properties:
- **scope**: Defines the visibility gradient (public, cohort, private)
- **commerce_action**: Reference to the commerce event being attested
- **settlement_hash**: Commitment to on-chain transaction data
- **timestamp**: When the receipt was recorded
- **actor**: The entity attesting to the commerce action
- **signature**: Cryptographic proof of attestation

### Usage:
Receipts provide a mechanism to optionally link off-chain commerce actions to their on-chain settlements while respecting the visibility gradient - they can be made public for auditability or kept private within cohort boundaries.

### Example: