#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

/// On-chain record for a single mineral lot certification.
///
/// `mineral` encodes the 3TG type: 1 = tin, 2 = tantalum, 3 = tungsten,
/// 4 = gold. `valid_until` is a future ledger sequence after which the
/// cert is considered expired. `status` mirrors the contract-wide
/// convention: 1 = valid, 2 = revoked (expiration is computed at verify
/// time). `reason` is a short, human-readable note set at issue / revoke.
#[contracttype]
#[derive(Clone)]
pub struct CertData {
    pub auditor: Address,
    pub smelter: Address,
    pub mineral: u32,
    pub valid_until: u32,
    pub status: u32,
    pub reason: Symbol,
}

/// Storage key namespace for the contract.
#[contracttype]
pub enum CertKey {
    /// Cert metadata keyed by lot id.
    Cert(Symbol),
    /// Set of lot ids registered under a particular smelter, used to
    /// quickly count how many lots a given refiner has produced.
    Lots(Address),
}

/// Mineral type encoding used throughout the contract.
pub const MINERAL_TIN: u32 = 1;
pub const MINERAL_TANTALUM: u32 = 2;
pub const MINERAL_TUNGSTEN: u32 = 3;
pub const MINERAL_GOLD: u32 = 4;

/// Cert status codes returned by `verify` and stored on-chain.
pub const STATUS_VALID: u32 = 1;
pub const STATUS_REVOKED: u32 = 2;

#[contract]
pub struct ConflictFreeCert;

#[contractimpl]
impl ConflictFreeCert {
    /// Issue a conflict-free certification for a mineral lot.
    ///
    /// The `auditor` (e.g. an RMI-approved third party) authorizes the
    /// call, binds a smelter/refiner `smelter` to a `lot_id`, records the
    /// 3TG `mineral` type, and sets a future-ledger expiry `valid_until`.
    /// The lot is also indexed under the smelter for later enumeration.
    pub fn issue_cert(
        env: Env,
        auditor: Address,
        smelter: Address,
        lot_id: Symbol,
        mineral: u32,
        valid_until: u32,
    ) {
        auditor.require_auth();

        if !(MINERAL_TIN..=MINERAL_GOLD).contains(&mineral) {
            panic!("invalid mineral code");
        }
        if valid_until <= env.ledger().sequence() {
            panic!("valid_until must be a future ledger");
        }

        let cert = CertData {
            auditor: auditor.clone(),
            smelter: smelter.clone(),
            mineral,
            valid_until,
            status: STATUS_VALID,
            reason: Symbol::new(&env, "issued"),
        };
        env.storage()
            .instance()
            .set(&CertKey::Cert(lot_id.clone()), &cert);

        let mut lots: Map<Symbol, bool> = env
            .storage()
            .instance()
            .get(&CertKey::Lots(smelter.clone()))
            .unwrap_or(Map::new(&env));
        lots.set(lot_id, true);
        env.storage()
            .instance()
            .set(&CertKey::Lots(smelter), &lots);
    }

    /// Revoke a previously issued certification, e.g. on audit failure.
    ///
    /// Only the auditor that originally issued the cert may revoke it.
    /// `reason` is recorded on-chain so importers and downstream parties
    /// can see why a lot was struck off the conflict-free list.
    pub fn revoke_cert(env: Env, auditor: Address, lot_id: Symbol, reason: Symbol) {
        auditor.require_auth();

        let key = CertKey::Cert(lot_id.clone());
        let mut cert: CertData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("cert not found"));

        if cert.auditor != auditor {
            panic!("only issuing auditor may revoke");
        }
        if cert.status == STATUS_REVOKED {
            panic!("already revoked");
        }

        cert.status = STATUS_REVOKED;
        cert.reason = reason;
        env.storage().instance().set(&key, &cert);
    }

    /// Verify a lot's certification status.
    ///
    /// Returns:
    /// * `0` — no certification on-chain for this lot
    /// * `1` — valid (issued/renewed and not past `valid_until`)
    /// * `2` — revoked by the auditor
    /// * `3` — expired (current ledger sequence is past `valid_until`)
    pub fn verify(env: Env, lot_id: Symbol) -> u32 {
        let cert: CertData = match env.storage().instance().get(&CertKey::Cert(lot_id)) {
            Some(c) => c,
            None => return 0,
        };

        if cert.status == STATUS_REVOKED {
            return 2;
        }
        if env.ledger().sequence() > cert.valid_until {
            return 3;
        }
        STATUS_VALID
    }

    /// Return the 3TG mineral type assigned to a lot.
    ///
    /// Returns `0` when no certification exists for `lot_id`, otherwise
    /// one of `1` (tin), `2` (tantalum), `3` (tungsten), `4` (gold).
    pub fn get_mineral(env: Env, lot_id: Symbol) -> u32 {
        let cert: CertData = match env.storage().instance().get(&CertKey::Cert(lot_id)) {
            Some(c) => c,
            None => return 0,
        };
        cert.mineral
    }

    /// Count how many distinct lots a smelter/refiner has registered.
    ///
    /// Importers and regulators can use this to gauge a smelter's
    /// certification footprint without iterating on-chain storage.
    pub fn list_lots(env: Env, smelter: Address) -> u32 {
        let lots: Map<Symbol, bool> = env
            .storage()
            .instance()
            .get(&CertKey::Lots(smelter))
            .unwrap_or(Map::new(&env));
        lots.len()
    }

    /// Renew an existing certification with a new future expiry.
    ///
    /// Useful when a smelter re-audits successfully and the cert is
    /// extended without re-issuing. Revoked certs cannot be renewed and
    /// must be re-issued from scratch.
    pub fn renew_cert(env: Env, auditor: Address, lot_id: Symbol, new_valid_until: u32) {
        auditor.require_auth();

        let key = CertKey::Cert(lot_id);
        let mut cert: CertData = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic!("cert not found"));

        if cert.auditor != auditor {
            panic!("only issuing auditor may renew");
        }
        if cert.status == STATUS_REVOKED {
            panic!("cannot renew a revoked cert");
        }
        if new_valid_until <= env.ledger().sequence() {
            panic!("new_valid_until must be a future ledger");
        }

        cert.valid_until = new_valid_until;
        cert.status = STATUS_VALID;
        cert.reason = Symbol::new(&env, "renewed");
        env.storage().instance().set(&key, &cert);
    }
}
