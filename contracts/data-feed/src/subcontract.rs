use loam_sdk::{
    soroban_sdk::{self, Address, Lazy, Vec},
    subcontract,
};

//quoted asset definition
#[loam_sdk::soroban_sdk::contracttype]
pub enum Asset {
    /// Can be a Stellar Classic or Soroban asset
    Stellar(loam_sdk::soroban_sdk::Address),
    /// For any external tokens/assets/symbols
    Other(loam_sdk::soroban_sdk::Symbol),
}

/// Price record definition
#[loam_sdk::soroban_sdk::contracttype]
pub struct PriceData {
    price: i128,    //asset price at given point in time
    timestamp: u64, //recording timestamp
}

#[subcontract]
/// Oracle Consumer Interface from https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0040.md
pub trait IsSep40 {
    /// Return all assets quoted by the price feed
    fn assets(&self) -> loam_sdk::soroban_sdk::Vec<Asset>;

    /// Return the base asset the price is reported in
    fn base(&self) -> Asset;

    /// Return the number of decimals for all assets quoted by the oracle
    fn decimals(&self) -> u32;

    /// Get the most recent price for an asset
    fn lastprice(&self, asset: Asset) -> Option<PriceData>;

    /// Get price in base asset at specific timestamp
    fn price(&self, asset: Asset, timestamp: u64) -> Option<PriceData>;

    /// Get last N price records
    fn prices(&self, asset: Asset, records: u32) -> Option<Vec<PriceData>>;

    /// Return default tick period timeframe (&self, in seconds)
    fn resolution(&self) -> u32;
}

#[subcontract]
/// SEP40 extension implemented by Reflector Network: https://reflector.network/docs/interface
pub trait IsReflector {
    /// Get the most recent price update timestamp
    fn last_timestamp(&self) -> u64;

    /// Get the most recent cross price record for the pair of assets
    fn x_last_price(&self, base_asset: Asset, quote_asset: Asset) -> Option<PriceData>;

    /// Get the cross price for the pair of assets at specific timestamp
    fn x_price(&self, base_asset: Asset, quote_asset: Asset, timestamp: u64) -> Option<PriceData>;

    /// Get last N cross price records of for the pair of assets
    fn x_prices(
        &self,
        base_asset: Asset,
        quote_asset: Asset,
        records: u32,
    ) -> Option<Vec<PriceData>>;

    /// Get the time-weighted average price for the given asset over N recent records
    fn twap(&self, asset: Asset, records: u32) -> Option<i128>;

    /// Get the time-weighted average cross price for the given asset pair over N recent records
    fn x_twap(&self, base_asset: Asset, quote_asset: Asset, records: u32) -> Option<i128>;

    /// Get historical records retention period, in seconds
    fn period(&self) -> Option<u64>;

    // Part of Reflector Network implementation, but omitted here because this is implemented via
    // the SmartDeploy / Loam Deploy Wasm package management infrastructure.
    // /// Get contract protocol version
    // fn version(&self, ) -> u32;

    // Part of the Reflector Network implementation, but ommitted here because `get_admin` is
    // already implemented by the `Core` subcontract.
    // /// Get contract admin address
    // fn admin(&self, ) -> Option<Address>;
}

#[subcontract]
/// While not part of the official consumer-facing spec, every SEP40/Reflector contract will need
/// to provide a way for Oracles to update the contract with new prices. This is an interface for
/// that, and also for other administrative functions, like initializing the contract.
///
/// Since Reflector's contract is a superset of SEP40, this admin interface is suitable for use
/// with either one. It contains extra fields, such as `retention_period`, which are not useful to
/// a strict implementation of SEP40.
pub trait IsReflectorAdmin {
    /// Initialize the contract with the given configuration.
    ///
    /// # Panics
    ///
    /// - if contract has already been initialized
    /// - if `admin_set` has not yet been called and there is therefore not yet an admin
    /// - if admin did not sign the transaction envelope
    fn reflector_init(
        &self,
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
    );
    /// Adds given assets to the contract quoted assets list. Can be invoked only by the admin account.
    ///
    /// # Arguments
    ///
    /// * `admin` - Admin account address
    /// * `assets` - Assets to add
    /// * `version` - Configuration protocol version
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match admin address, or if the assets are already added
    fn add_assets(&mut self, assets: Vec<Asset>);
}
