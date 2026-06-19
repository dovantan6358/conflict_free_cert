# conflict_free_cert

## Project Title
conflict_free_cert

## Project Description
conflict_free_cert is a Soroban smart contract that brings transparent, on-chain accountability to the 3TG conflict-mineral supply chain (tin, tantalum, tungsten and gold). Independent auditors can issue tamper-evident certifications for individual mineral lots produced by registered smelters and refiners, importers can verify a lot's status in a single read, and certifications can be revoked on the same ledger when an audit fails — eliminating the paper-trail and PDF-attestation gap that exists in current due-diligence workflows.

## Project Vision
Our long-term vision is a globally queryable, regulator-friendly ledger of conflict-free mineral provenance that any downstream manufacturer, OEM or customs office can consult in real time. By turning certifications into signed, revocable, time-bound ledger entries on Stellar, we aim to make "conflict-free" an attribute that can be cryptographically proven at the moment a metal lot changes hands, not asserted weeks later in a self-declared questionnaire.

## Key Features
- **Auditor-signed issuance** — only the calling auditor can bind a smelter, lot id, 3TG mineral type and expiry into a single `CertData` record via `issue_cert`.
- **Public, on-chain verification** — `verify` returns a four-state status (none, valid, revoked, expired) that any importer, OEM or customs system can call with just a `lot_id`.
- **Revocable certifications** — auditors can strike a lot off the conflict-free list with `revoke_cert`, recording the reason on-chain for full traceability.
- **Renewal without re-issuance** — `renew_cert` extends a still-trustworthy cert to a new future ledger sequence while preserving its original auditor and lot binding.
- **Per-smelter lot index** — `list_lots` exposes the size of a smelter's certified footprint for regulator dashboards and supplier scorecards.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** supply_chain dApp — see `contracts/conflict_free_cert/src/lib.rs` for the full conflict_free_cert business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `<CC5QNP5VDUA62PMVZTEPYOYKLWAL3H777QLAIJBYIJOEGJUZTFUC5BLR>`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/427920b832608d08e37087c850d3658e7824be9be24e3f684698aef0c9eafa31>`
- **Screenshot of deployed contract on Stellar Expert:**
  `_(https://prnt.sc/zFmHDHApWSD2)_`


## Future Scope
- Multi-auditor co-signature scheme (M-of-N) for high-risk lots such as coltan from the Great Lakes region.
- Off-chain IPFS / Content-Addressed Storage hash of the full audit report attached to each `CertData` entry.
- Royalty / fee split paid in XLM to the auditor on successful renewal, integrated via Stellar's native asset interface.
- Per-mineral (3TG) aggregate statistics and a public dashboard powered by ledger event streaming.
- Cross-chain bridge hooks so that downstream Soroban tokenized-metal contracts can consume the `verify` status as an on-chain compliance oracle.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `conflict_free_cert` (supply_chain)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
