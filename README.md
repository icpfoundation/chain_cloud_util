# chain_cloud_util
  chain_cloud_util stores records that other canisters want to store, similar to Ethereum event, but chain_cloud_util stores data in event canister.

## take care
  Note that at present, event canisters do not exist on the main network, and will be deployed on the main network soon. Please pay continuous attention
## How to use
### example
    use ic_cdk_macros::*;
    use context::event::EventTrait;
    use context::event_macro::Event;
    use ic_cdk::export::Principal;
    use ic_cdk::export::candid::Nat;
    use ic_cdk::api;
    use context::emit;
    #[derive(Event)]
    struct MintEvent{
        method_name:String,
        memo:String,
    }

    #[derive(Event)]
    struct TransferEvent{
        method_name:String,
        memo:String,
    }
    
    #[update]
    async fn mint() ->() {
        ic_cdk::print("mint");
        let mint_event = MintEvent{
            method_name:"mint".to_string(),
            memo:"mint token".to_string(),
        }; 
        emit(mint_event).await;
    }

    #[update]
    async fn transfer() ->() {
        ic_cdk::print("transfer"); 
        let transfer_event = TransferEvent{
            method_name:"transfer".to_string(),
            memo:"transfer token".to_string(),
        };
        emit(transfer_event).await;
    }

## Parameter meaning
    method_name: The function name of the currently executing function
    memo: The length of some information you think important is limited