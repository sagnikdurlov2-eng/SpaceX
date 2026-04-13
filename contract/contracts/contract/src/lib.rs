#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Address, Map, Symbol, String,
};

#[derive(Clone)]
#[contracttype]
pub struct SpaceAsset {
    pub owner: Address,
    pub name: String,
}

#[contract]
pub struct SpaceRegistry;

#[contractimpl]
impl SpaceRegistry {

    // Storage key
    fn storage_key() -> Symbol {
        symbol_short!("ASSETS")
    }

    // Get stored assets
    fn get_assets(env: &Env) -> Map<u32, SpaceAsset> {
        env.storage()
            .instance()
            .get(&Self::storage_key())
            .unwrap_or(Map::new(env))
    }

    // Save assets
    fn set_assets(env: &Env, assets: &Map<u32, SpaceAsset>) {
        env.storage().instance().set(&Self::storage_key(), assets);
    }

    // Register new asset
    pub fn register_asset(env: Env, asset_id: u32, owner: Address, name: String) {
        owner.require_auth();

        let mut assets = Self::get_assets(&env);

        if assets.contains_key(asset_id) {
            panic!("Asset already exists");
        }

        let asset = SpaceAsset { owner, name };

        assets.set(asset_id, asset);
        Self::set_assets(&env, &assets);
    }

    // Transfer ownership
    pub fn transfer_asset(env: Env, asset_id: u32, from: Address, to: Address) {
        from.require_auth();

        let mut assets = Self::get_assets(&env);

        let mut asset = match assets.get(asset_id) {
            Some(a) => a,
            None => panic!("Asset not found"),
        };

        if asset.owner != from {
            panic!("Not the owner");
        }

        asset.owner = to;

        assets.set(asset_id, asset);
        Self::set_assets(&env, &assets);
    }

    // Get asset details
    pub fn get_asset(env: Env, asset_id: u32) -> SpaceAsset {
        let assets = Self::get_assets(&env);

        match assets.get(asset_id) {
            Some(asset) => asset,
            None => panic!("Asset not found"),
        }
    }
}