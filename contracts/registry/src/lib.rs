#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

// ═══════════════════════════════════════════════════════════════════════════
// STORAGE TYPES (issue 2)
// ═══════════════════════════════════════════════════════════════════════════

#[contracttype]
#[derive(Clone)]
pub struct Attestation {
    pub id: u64,
    pub contributor: Address,
    pub repo_url: String,
    pub timestamp: u64,
    pub revoked: bool,
    pub revocation_timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct RepoBinding {
    pub repo_url: String,
    pub owner: Address,
    pub bound_at: u64,
}

#[contracttype]
pub enum DataKey {
    Attestation(u64),
    ContributorAttestations(Address),
    RepoBinding(String),
    AttestationCounter,
    Admin,
}

// ═══════════════════════════════════════════════════════════════════════════
// CONTRACT IMPLEMENTATION (issues 3-8)
// ═══════════════════════════════════════════════════════════════════════════

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    // Issue 8: Admin initialization
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::AttestationCounter, &0u64);
    }

    // Issue 3: attest() function
    pub fn attest(env: Env, contributor: Address, repo_url: String) -> u64 {
        contributor.require_auth();
        let counter: u64 = env.storage().instance().get(&DataKey::AttestationCounter).unwrap_or(0);
        let id = counter + 1;
        let attestation = Attestation {
            id,
            contributor: contributor.clone(),
            repo_url: repo_url.clone(),
            timestamp: env.ledger().timestamp(),
            revoked: false,
            revocation_timestamp: 0,
        };
        env.storage().persistent().set(&DataKey::Attestation(id), &attestation);
        env.storage().instance().set(&DataKey::AttestationCounter, &id);
        
        let mut ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::ContributorAttestations(contributor.clone()))
            .unwrap_or(Vec::new(&env));
        ids.push_back(id);
        env.storage().persistent().set(&DataKey::ContributorAttestations(contributor), &ids);
        
        env.events().publish((Symbol::new(&env, "attest"),), (id, repo_url));
        id
    }

    // Issue 4: revoke() function  
    pub fn revoke(env: Env, caller: Address, attestation_id: u64) {
        caller.require_auth();
        let mut attestation: Attestation = env.storage().persistent()
            .get(&DataKey::Attestation(attestation_id))
            .expect("attestation not found");
        if attestation.contributor != caller {
            let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("no admin");
            if caller != admin { panic!("unauthorized"); }
        }
        attestation.revoked = true;
        attestation.revocation_timestamp = env.ledger().timestamp();
        env.storage().persistent().set(&DataKey::Attestation(attestation_id), &attestation);
        env.events().publish((Symbol::new(&env, "revoke"),), (attestation_id,));
    }

    // Issue 5: get_attestation() query function
    pub fn get_attestation(env: Env, attestation_id: u64) -> Attestation {
        env.storage().persistent().get(&DataKey::Attestation(attestation_id))
            .expect("attestation not found")
    }

    // Issue 6: list_attestations() for contributor
    pub fn list_attestations(env: Env, contributor: Address) -> Vec<u64> {
        env.storage().persistent().get(&DataKey::ContributorAttestations(contributor))
            .unwrap_or(Vec::new(&env))
    }

    // Issue 7: repo_binding contract
    pub fn bind_repo(env: Env, owner: Address, repo_url: String) {
        owner.require_auth();
        let binding = RepoBinding {
            repo_url: repo_url.clone(),
            owner: owner.clone(),
            bound_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&DataKey::RepoBinding(repo_url.clone()), &binding);
        env.events().publish((Symbol::new(&env, "bind_repo"),), (owner, repo_url));
    }

    pub fn get_binding(env: Env, repo_url: String) -> RepoBinding {
        env.storage().persistent().get(&DataKey::RepoBinding(repo_url))
            .expect("binding not found")
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("no admin")
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS (issue 9)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_initialize_and_attest() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, RegistryContract);
        let client = RegistryContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let contributor = Address::generate(&env);
        client.initialize(&admin);
        let repo = String::from_str(&env, "https://github.com/test/repo");
        let id = client.attest(&contributor, &repo);
        assert_eq!(id, 1);
        let attestation = client.get_attestation(&id);
        assert_eq!(attestation.contributor, contributor);
        assert!(!attestation.revoked);
    }

    #[test]
    fn test_revoke() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, RegistryContract);
        let client = RegistryContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let contributor = Address::generate(&env);
        client.initialize(&admin);
        let repo = String::from_str(&env, "https://github.com/test/repo");
        let id = client.attest(&contributor, &repo);
        client.revoke(&contributor, &id);
        let attestation = client.get_attestation(&id);
        assert!(attestation.revoked);
    }

    #[test]
    fn test_bind_repo() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, RegistryContract);
        let client = RegistryContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        client.initialize(&admin);
        let repo = String::from_str(&env, "https://github.com/test/repo");
        client.bind_repo(&owner, &repo);
        let binding = client.get_binding(&repo);
        assert_eq!(binding.owner, owner);
    }
}
