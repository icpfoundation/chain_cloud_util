pub mod metadata;
pub mod util;

mod config;
use config::{CREATETRANSACTION, EVENTCANISTER};
use event::EventTrait;
use event_macro::Event;
use ic_cdk::api;
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use metadata::Metadata;
pub extern crate event;
pub extern crate event_macro;

/// Listen to events, transfer reasonable data to event canisters and store them
pub async fn emit(event: impl EventTrait) -> CallResult<()> {
    if event.method_name() == "" {
        return Err((RejectionCode::Unknown, "method_name is empty".to_string()));
    }
    // Note that the remarks should not be too long, which will bring a great burden to the event canisters during message delivery
    if event.memo().len() > 30 {
        return Err((RejectionCode::Unknown, "memo too long".to_string()));
    }
    let canister_id = event.canister_id();
    let caller_id = event.caller_id();
    let event_time = event.event_create_time();
    let cycle = event.canister_balance();
    let stable_size = event.stable_size();
    let method_name = event.method_name();
    let memo = event.memo();
    let new_metadata = Metadata::new(
        &canister_id,
        &caller_id,
        event_time.into(),
        stable_size.into(),
        cycle.into(),
        &method_name,
        &memo,
    );
    match Principal::from_text(EVENTCANISTER) {
        Ok(event_canister_id) => {
            api::call::call(event_canister_id, CREATETRANSACTION, (&new_metadata,)).await
        }
        Err(err) => Err((RejectionCode::Unknown, err.to_string())),
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
