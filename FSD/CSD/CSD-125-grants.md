# CSD-125 — Grants (access to one thing, from its owner, and never passed on)

**CSD**: CSD-125 · **Standard**: CSD/3 (`CIRISClient/CSD.md`) · **Origin**: the registry fold, FSD-005; CIRISBilling resource grants
**Flow**: unwritten
**Reads with**: CSD-124 (a `grant`-scoped delegate issues on the owner's behalf), CSD-053/054 (consent, the person's own grants over their data), CSD-056 (billing, off the wire), CSD-107 (file custody)

```yaml csd:stage
stage: envisioned
owner: CIRISRegistry
```

## 1. Mission (why)

**The owner or steward of a thing — a sealed file, a stream, a community epoch,
a metered resource — can grant a named recipient access to it, for a window, see
every grant that is live over what they own, and see what a recipient holds;
and nobody who received a grant can pass it on.** Serves **Contextual
Integrity**, on CC 2.4.1.2.1: "a grant is non-transferable by default … an onward
grant is a new grant issued by the asset's owner or steward, or by a holder of a
`grant`-scoped delegation from them, never a forwarded one", and "possession of
the data-encryption key is not issuance authority."

**Two wire shapes, one card.** A key wrap is `key_grant:{axis}:v1`, one scores
row per recipient (rc7, CIRISConstitution#143), `{axis}` ∈ content | epoch |
stream, sealer-only, never withdrawn (forward secrecy is by rotation). A consent
grant is `consent:scope:{kind}` over a person's data. Both answer "who may reach
this thing", both are leaves of the authority tree, and the card renders them in
the same rows. Resource grants for CIRISBilling (a metered allowance) are the
same shape over a `settlement:*` / `ledger:*` subject and are **not specified
here** until the Constitution names the family (CIRISRegistry#51 / CC 3.3.10).

**A grant is never withdrawn and the card never offers it.** CC 3.1.3: a
`withdraws` naming a `key_grant` row is refused. Ending access is rotation: a
new epoch, a new DEK, and the recipient is not in the new wrap set. The card's
"end access" is therefore "rotate and exclude", and it says so.

## 2. Surface (what)

```yaml csd:surface
surface: proposed:grants
screen: proposed:Grants
```

Placement: My things › Files (grants over my content) and Communities and
Businesses › Rules (grants over the affiliation's assets). The same screen, two
scopes.

```yaml csd:shows
registry_sha256: 796f609981434b838edab1165462484f1be67889eed7f60c26b210ede5984464
fields:
  - ceg: "key_grant:{axis}"
    bind: {axis: content}
    use: display-only
    type: "enum[content,epoch,stream]"
    example: "content"
    renders: "one row per (thing, recipient): 'invoice.pdf — shared with {who}' / 'the Allotment room, epoch 7 — {who}' / 'the Tuesday stream — {who}'"
    tag: "proposed:row_grant_*"
  - ceg: x_private:recipient_key_id
    use: emit
    type: string
    example: "wa-peer-4a19c2"
    renders: "'To' — the recipient's identity occurrence; picked from contacts or the roster, never typed as an email"
    tag: "proposed:input_grant_recipient"
  - ceg: x_private:asset
    use: emit
    type: string
    example: "at_rest_sha256:5ef9e607…"
    renders: "'What' — the thing, named by the identity on its axis (the blob digest, the community + epoch, the stream id)"
    tag: "proposed:txt_grant_asset_*"
  - ceg: x_private:issued_by
    use: display-only
    type: string
    example: "wa-self-88b1"
    renders: "'Granted by' — the sealer; 'under a grant-scoped delegation from {owner}' when the issuer is a delegate (CSD-124)"
    tag: "proposed:txt_grant_issued_by_*"
  - ceg: x_private:key_validity_window
    use: emit
    type: timestamp
    example: "2026-12-31T00:00:00Z"
    renders: "'Until' — optional; a grant past its window opens nothing and the row says 'expired'"
    tag: "proposed:input_grant_until"
  - ceg: x_private:wrap_algorithm
    use: display-only
    type: "enum[v2]"
    example: "v2"
    renders: "'Post-quantum wrapped' — v2 only (CC 4.4.3.4.1); a v1 wrap is shown as refused, never as a grant"
    tag: "proposed:chip_grant_wrap_*"
  - ceg: x_private:onward_grant_refused
    use: display-only
    type: bool
    example: true
    renders: "on a grant I RECEIVED: no 'share' control at all, and the explainer 'You can open this. You cannot pass it on; ask {owner}.' — receipt is not issuance"
    tag: "proposed:txt_grant_no_onward_*"
  - ceg: x_private:end_access_is_rotation
    use: emit
    type: bool
    example: true
    renders: "'End access' on a grant I ISSUED: the confirm says 'This rotates the key. {who} keeps what they already fetched and gets nothing new.' — never 'revoke'"
    tag: "proposed:btn_grant_end_*"
  - ceg: "consent:scope:{kind}"
    bind: {kind: share}
    use: display-only
    type: string
    example: "consent:scope:share:v1"
    renders: "a consent grant over my data in the same list, with its kind; emitted from CSD-053/054, read here"
    tag: "proposed:row_consent_grant_*"
```

**`consent:scope:*` is reserved to CIRISAgent at CC 3.4.5**, so this card reads
it and does not emit it; the emitting card is CSD-053/054. `key_grant:*` is
sealer-only (CIRISPersist's reservation): the card emits by asking the node to
seal, never by composing a wrap itself.

```yaml csd:states
populated: {tag: "proposed:list_grants"}
empty:     {tag: "proposed:txt_grants_empty", renders: "Nothing shared yet. / Nothing has been shared with you that this node holds the key for."}
loading:   {renders: "the frame with a progress affordance and NO sentence"}
error:     {tag: "proposed:txt_grants_error", renders: "Couldn't read grants from this node, so this list is not complete."}
```

## 3. Contracts (who)

| What | Route | Owner | State |
|---|---|---|---|
| grant access to a file (the node seals a wrap to the recipient) | partial: `POST /v1/files` at a cohort places it for a roster, not for one recipient | CIRISServer | `blocked_by: FSD-005 §7` |
| read grants I issued / grants I hold | none as a list (drive rows carry custody, CSD-107) | CIRISServer | `blocked_by: FSD-005 §7` |
| end access = rotate and exclude | partial (room epoch rotation exists for communities) | CIRISServer | unconfirmed |
| a `grant`-scoped delegate may seal on the owner's behalf; admission refuses a recipient re-granting | CIRISPersist | CIRISPersist | unconfirmed — FSD-005 §7 |
| a metered resource grant (CIRISBilling) | no family | CIRISConstitution | `blocked_by: CIRISRegistry#51` |

## 4. Flow

Unwritten at `envisioned`. The chain: owner grants A → A opens it → A's card has
no share control → owner delegates `grant` to B (CSD-124) → B grants C → C's row
names B under the owner's delegation → owner ends access: rotation; A and C open
nothing new.

## 5. Acceptance

Unsigned. Not tested here: the consent grants' own emission (CSD-053/054), the
bytes' custody (CSD-107), billing (CSD-056).
