#![no_std]
use loam_sdk::{derive_contract, soroban_sdk::Vec};
use loam_subcontract_core::{admin::Admin, Core};

mod contract;

pub use contract::Landscape;
