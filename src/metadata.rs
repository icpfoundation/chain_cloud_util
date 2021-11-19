use ic_cdk::export::candid::CandidType;
use ic_cdk::export::candid::{Deserialize, Nat};
use ic_cdk::export::Principal;

#[derive(Deserialize, Debug, Clone, CandidType)]
pub struct Metadata {
    /// canister ID that created the event
    pub canister: Principal,
    /// Account number of the current operation
    pub caller: Principal,
    /// Current canister cycle balance
    pub cycle: Nat,
    /// Name of the currently executed function
    pub method_name: String,
    /// current time
    pub transaction_time: Nat,
    /// Memory size currently used by canisters
    pub stable_size: Nat,
    /// Note: you can fill in at will, but the length cannot exceed 30
    pub memo: String,
}

impl Metadata {
    pub fn new(
        canister: &Principal,
        caller: &Principal,
        transaction_time: Nat,
        stable_size: Nat,
        cycle: Nat,
        method_name: &str,
        memo: &str,
    ) -> Self {
        Self {
            canister: canister.clone(),
            caller: caller.clone(),
            cycle: cycle,
            method_name: method_name.to_string(),
            transaction_time: transaction_time,
            stable_size: stable_size,
            memo: memo.to_string(),
        }
    }
}
