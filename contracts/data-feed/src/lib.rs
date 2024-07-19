#![no_std]
use loam_sdk::{derive_contract, soroban_sdk::Vec};
use loam_subcontract_core::{admin::Admin, Core};

pub mod data_feed;
pub mod subcontract;

use data_feed::DataFeed;
use subcontract::{Asset, PriceData, Reflector, ReflectorAdmin, Sep40};

#[derive_contract(
    Core(Admin),
    // Sep40(DataFeed),
    // Reflector(DataFeed),
    ReflectorAdmin(DataFeed)
)]
pub struct Contract;

impl Contract {
    pub(crate) fn require_auth() {
        Contract::admin_get()
            .expect("No admin! Call 'admin_set' first.")
            .require_auth();
    }
}
