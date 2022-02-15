pub mod types;
pub mod util;

mod config;
use config::{CREATETRANSACTION, EVENTCANISTER};
use event::EventTrait;
use event_macro::Event;
use ic_cdk::api;
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use types::Log;
pub extern crate event;
pub extern crate event_macro;

/// Listen to events, transfer reasonable data to event canisters and store them
pub async fn emit(event: impl EventTrait) -> CallResult<()> {
    let canister_id = event.canister_id();
    let caller_id = event.caller_id();
    let event_time = event.event_create_time();
    let cycle = event.canister_balance();
    let stable_size = event.stable_size();
    let method_name = event.method_name();
    let memo = event.memo();
    let new_log = Log::new(
        &canister_id,
        &caller_id,
        event_time.into(),
        stable_size.into(),
        cycle.into(),
        &method_name,
        &memo,
    );
    match new_log {
        Ok(value) => match Principal::from_text(EVENTCANISTER) {
            Ok(event_canister_id) => {
                api::call::call(event_canister_id, CREATETRANSACTION, (&value,)).await
            }
            Err(err) => Err((RejectionCode::Unknown, err.to_string())),
        },
        Err(err) => {
            return Err((RejectionCode::Unknown, err.to_string()));
        }
    }
}

#[cfg(test)]
mod event_macro_test {
    use super::*;

    #[derive(Event)]
    struct MintEventTest {
        pub method_name: String,
        pub memo: String,
    }
    #[test]
    fn event_trait_test() {
        let mint_event = MintEventTest {
            method_name: "mint".to_string(),
            memo: "memo".to_string(),
        };
        let method_name = mint_event.method_name();
        println!("method_name {}", method_name);
        let memo = mint_event.memo();
        println!("memo {}", memo);
    }
}
