use ic_cdk::export::candid::CandidType;
use ic_cdk::export::candid::{Deserialize, Nat};
use ic_cdk::export::Principal;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LogErr {
    LengthErr(String),
}

impl fmt::Display for LogErr{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LogErr::LengthErr(err) =>{
               return  write!(f,"{}",err)
            }
        }
    }
}
impl Error for LogErr {
    fn description(&self) -> &str {
        match self{
            LogErr::LengthErr(err) =>{
               return  err
            }
        }
    }
}


#[derive(Deserialize, Debug, Clone, CandidType)]
pub struct Log {
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


impl Log{
    pub fn new(
        canister: &Principal,
        caller: &Principal,
        transaction_time: Nat,
        stable_size: Nat,
        cycle: Nat,
        method_name: &str,
        memo: &str,
    ) -> Result<Self,LogErr> {
        if method_name == "" {
            return Err(LogErr::LengthErr("method_name is empty".to_string()));
        }
        // Note that the remarks should not be too long, which will bring a great burden to the event canisters during message delivery
        if memo.len() > 150 {
            return Err(LogErr::LengthErr("memo too long".to_string()));
        }
    
       Ok(Self {
            canister: canister.clone(),
            caller: caller.clone(),
            cycle: cycle,
            method_name: method_name.to_string(),
            transaction_time: transaction_time,
            stable_size: stable_size,
            memo: memo.to_string(),
        })
    }
}
