#[subcontract]
pub trait CDP {
    // Deploy new xAsset contract (so it will need to store the wasm of the xAsset contract, or a reference to it by name on the Loam Registry), which is only callable by admin.

    fn create_contract(&self, asset: Asset, contract: Address);

    // List all CDPs for a given account, so it can iterate the "map of asset names" keys and make cross-contract calls to see if the given account has an entry in its CDPs map.
    fn list_cdps(&self, account: Address);
}
