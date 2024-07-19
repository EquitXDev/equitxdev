use loam_sdk::{
    soroban_sdk::{self, contracttype, env, vec, Address, Bytes, Lazy, Map, Vec},
    IntoKey,
};
use loam_subcontract_core::Core;

use crate::subcontract::{Asset, IsReflector, IsReflectorAdmin, IsSep40, PriceData};
use crate::Contract;

#[contracttype]
#[derive(IntoKey)]
pub struct DataFeed {
    assets: Map<Asset, ()>,
    base: Asset,
    decimals: u32,
    // price: Map<(Asset, u64), PriceData>,
    // prices: Map<(Asset, u32), Vec<PriceData>>,
    resolution: u32,
    last_timestamp: u64,
    retention_period: u64,
    // x_last_price: Map<(Asset, Asset), PriceData>,
    // x_price: Map<(Asset, Asset, u64), PriceData>,
    // x_prices: Map<(Asset, Asset, u32), Vec<PriceData>>,
}

impl DataFeed {
    #[must_use]
    pub fn new(
        // The assets supported by the contract.
        assets: Vec<Asset>,
        // The base asset for the prices.
        base: Asset,
        // The number of decimals for the prices.
        decimals: u32,
        // The resolution of the prices.
        resolution: u32,
        // The retention period for the prices.
        retention_period: u64,
    ) -> Self {
        let mut asset_map = Map::new(env());
        for asset in assets.into_iter() {
            asset_map.set(asset, ());
        }
        DataFeed {
            assets: asset_map,
            base,
            decimals,
            resolution,
            last_timestamp: 0,
            retention_period,
        }
    }
}

/// Loam SDK currently requires us to implement `Default`. This is nonsense and will be fixed in
/// https://github.com/loambuild/loam/issues/92
impl Default for DataFeed {
    fn default() -> Self {
        DataFeed::new(
            vec![env()],
            Asset::Stellar(env().current_contract_address()),
            0,
            0,
            0,
        )
    }
}

impl IsReflectorAdmin for DataFeed {
    fn reflector_init(
        &self,
        assets: Vec<Asset>,
        base: Asset,
        decimals: u32,
        resolution: u32,
        retention_period: u64,
    ) {
        Contract::require_auth();
        DataFeed::set_lazy(DataFeed::new(
            assets,
            base,
            decimals,
            resolution,
            retention_period,
        ));
    }

    fn add_assets(&mut self, assets: Vec<Asset>) {
        Contract::require_auth();
        for asset in assets {
            self.assets.set(asset, ());
        }
    }
}
