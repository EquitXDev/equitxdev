use loam_sdk::{
    soroban_sdk::{self, contracttype, env, xdr::Liabilities, Address, Bytes, Env, Lazy, Map, Vec},
    IntoKey,
};

use crate::cdp::CDP;

pub enum Asset {
    /// Can be a Stellar Classic or Soroban asset
    Stellar(loam_sdk::soroban_sdk::Address),
    /// For any external tokens/assets/symbols
    Other(loam_sdk::soroban_sdk::Symbol),
}

#[contracttype]
#[derive(IntoKey)]
pub struct Landscape {
    // A map of asset names ("xUSD") to contract addresses ("C123…")
    assets: Map<Asset, Address>,
}

impl Landscape {
    #[must_use]
    pub fn new(assets: Map<Asset, Address>) -> Self {
        let mut asset_map = Map::new(env());
        for asset in assets.into_iter() {
            asset_map.set(asset, Map::new(env()));
        }
        Landscape { assets: asset_map }
    }
}

/// Loam SDK currently requires us to implement `Default`. This will be fixed in
/// https://github.com/loambuild/loam/issues/92
impl Default for Landscape {
    fn default() -> Self {
        Landscape::new(Map::new(env()))
    }
}

impl CDP for Landscape {
    // Deploy new xAsset contract (so it will need to store the wasm of the xAsset contract, or a reference to it by name on the Loam Registry), which is only callable by admin.

    fn create_contract(&self, asset: Asset, contract: Address) {
        todo!()
        //self.assets.set(asset, contract);
    }

    // List all CDPs for a given account, so it can iterate the "map of asset names" keys and make cross-contract calls to see if the given account has an entry in its CDPs map.
    fn list_cdps(&self, account: Address) {
        todo!()
    }
}
