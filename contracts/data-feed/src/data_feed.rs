use loam_sdk::{
    soroban_sdk::{
        self, contracttype, env, panic_with_error, vec, Address, Bytes, Env, Lazy, Map, Vec,
    },
    IntoKey,
};

use crate::sep40::{Asset, IsSep40, IsSep40Admin, PriceData};
use crate::u64_extensions::U64Extensions;
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
        )
    }
}

impl IsSep40Admin for DataFeed {
    fn sep40_init(&self, assets: Vec<Asset>, base: Asset, decimals: u32, resolution: u32) {
        Contract::require_auth();
        DataFeed::set_lazy(DataFeed::new(assets, base, decimals, resolution));
    }

    fn add_assets(&mut self, assets: Vec<Asset>) {
        Contract::require_auth();
        for asset in assets {
            self.assets.set(asset, ());
        }
    }

    fn set_price(&mut self, updates: Vec<i128>, timestamp: u64) {
        Contract::require_auth();
        todo!();
        // let updates_len = updates.len();
        // if updates_len == 0 || updates_len >= 256 {
        //     panic!("The assets update length or prices update length is invalid");
        // }
        // let timeframe: u64 = self.resolution.into();
        // let ledger_timestamp = now();
        // if timestamp == 0
        //     || !timestamp.is_valid_timestamp(timeframe)
        //     || timestamp > ledger_timestamp
        // {
        //     panic!("The prices timestamp is invalid");
        // }
        //
        // // from reflector implementation
        // // let retention_period = e.get_retention_period();
        //
        // // let ledgers_to_live: u32 = ((retention_period / 1000 / 5) + 1) as u32;
        //
        // //iterate over the updates
        // for (i, price) in updates.iter().enumerate() {
        //     //don't store zero prices
        //     if price == 0 {
        //         continue;
        //     }
        //     let asset = i as u8;
        //     //store the new price
        //     e.set_price(asset, price, timestamp, ledgers_to_live);
        // }
        // if timestamp > self.last_timestamp {
        //     self.last_timestamp = timestamp;
        // }
    }
}

/// Get the timestamp from env, converted to milliseconds
fn now() -> u64 {
    env().ledger().timestamp() * 1000
}
