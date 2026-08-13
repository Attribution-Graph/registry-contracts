#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// ── Storage types ────────────────────────────────────────────────────────────

/// An on-chain attestation recording a contributor's claim against a repository.
#[contracttype]
#[derive(Clone)]
pub struct Attestation {
    /// Unique sequential identifier.
    pub id: u64,
    /// Stellar address of the contributor making the attestation.
    pub contributor: Address,
    /// GitHub repository URL being attested.
    pub repo_url: String,
    /// Ledger timestamp at creation.
    pub timestamp: u64,
    /// Whether this attestation has been revoked.
    pub revoked: bool,
    /// Ledger timestamp of revocation (0 if not revoked).
    pub revocation_timestamp: u64,
}

/// Binds a GitHub repository URL to a Stellar account.
#[contracttype]
#[derive(Clone)]
pub struct RepoBinding {
    pub repo_url: String,
    pub owner: Address,
    pub bound_at: u64,
}

/// Storage key enum covering all persistent/instance data.
#[contracttype]
pub enum DataKey {
    Attestation(u64),
    ContributorAttestations(Address),
    RepoBinding(String),
    AttestationCounter,
    Admin,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_data_key_variants_exist() {
        // Compile-time check: DataKey variants are accessible.
        let _counter = DataKey::AttestationCounter;
        let _admin = DataKey::Admin;
    }
}
