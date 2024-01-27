// a -> b
// a = benefactor
// b = beneficiary

// ./build.sh

// account
// near account create-account sponsor-by-faucet-service fairfungibletoken.testnet autogenerate-new-keypair save-to-keychain network-config testnet create

// deploy
// near contract deploy fairfungibletoken.testnet use-file target/wasm32-unknown-unknown/release/fft.wasm without-init-call network-config testnet sign-with-keychain

// view
// near contract view-storage fairfungibletoken.testnet all as-json network-config testnet

// register
// near contract call-function as-transaction ref.fakes.testnet storage_deposit json-args '{"account_id": "fairfungibletoken.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0.00125 NEAR' sign-as fairfungibletoken.testnet network-config testnet sign-with-keychain send

// create
// near contract call-function as-transaction ref.fakes.testnet ft_transfer_call json-args '{"receiver_id": "fairfungibletoken.testnet", "amount": "3000000000000000000", "msg": "1mm1.testnet,1706369099063000000,1706455499063000064,lin"}' prepaid-gas '100.0 Tgas' attached-deposit '0.000000000000000000000001 NEAR' sign-as 1m1.testnet network-config testnet sign-with-keychain send

// take
// near contract call-function as-transaction fairfungibletoken.testnet take json-args '{"a": "1m1.testnet", "b": "1mm1.testnet", "ft": "ref.fakes.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as 1m1.testnet network-config testnet sign-with-keychain send

// clear
// near contract call-function as-transaction fairfungibletoken.testnet clear json-args '{"b": "1mm1.testnet"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as 1m1.testnet network-config testnet sign-with-keychain send

// view
// near contract call-function as-read-only fairfungibletoken.testnet view json-args '{"a": "1m1.testnet", "b": "1mm1.testnet", "ft": "ref.fakes.testnet"}' network-config testnet now

// Find all our documentation at https://docs.near.org
use near_contract_standards::fungible_token::core::ext_ft_core::ext;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::log_str;
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use near_sdk::{env, PromiseOrValue};
use near_sdk::{AccountId, Balance};

use std::str::FromStr;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FFT {
    // b -> ft -> a -> schedule
    pub per_b: LookupMap<AccountId, LookupMap<AccountId, LookupMap<AccountId, Schedule>>>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum ScheduleType {
    Log, // todo
    Lin,
    Exp, // todo
}
impl Default for ScheduleType {
    fn default() -> Self {
        ScheduleType::Lin
    }
}
impl FromStr for ScheduleType {
    type Err = ();

    fn from_str(s: &str) -> Result<ScheduleType, ()> {
        match s {
            "log" => Ok(ScheduleType::Log),
            "lin" => Ok(ScheduleType::Lin),
            "exp" => Ok(ScheduleType::Exp),
            _ => Err(()),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Schedule {
    pub begin: u64,     // timestamp
    pub last_take: u64, // timestamp
    pub end: u64,       // timestamp
    pub available_balance: Balance,
    pub taken_balance: Balance,
    pub t: ScheduleType,
}

// Define the default, which automatically initializes the contract
impl Default for FFT {
    fn default() -> Self {
        Self {
            per_b: LookupMap::new(b"per_b".to_vec()),
        }
    }
}

fn str_to_account_id(s: &str) -> AccountId {
    s.parse().expect("Invalid AccountId")
}
fn convert(balance: u128) -> U128 {
    U128(balance)
}

// Implement the contract structure
#[near_bindgen]
impl FFT {
    // create or add
    pub fn ft_on_transfer(& mut self, sender_id: String, amount: String, msg: String) -> String {
        // log_str("hi");

        let a = str_to_account_id(&sender_id);

        let mut parts = msg.split(',');

        let b_str = parts.next().unwrap_or_default();
        // log_str(&format!("b_str: {b_str}"));
        let b = str_to_account_id(b_str);
        // log_str(&format!("b: {b}"));

        let begin_str = parts.next().unwrap_or_default();
        // log_str(&format!("begin_str: {begin_str}"));
        let begin: u64 = begin_str.parse().expect("begin_str not a valid timestamp");
        // log_str(&format!("begin: {begin}"));

        let end_str = parts.next().unwrap_or_default();
        let end: u64 = end_str.parse().expect("end_str not a valid timestamp");

        let t_str = parts.next().unwrap_or_default();
        let t = ScheduleType::from_str(t_str).unwrap_or_default();

        let ft = env::predecessor_account_id();

        let mut available_balance = 0;
        match amount.parse::<u128>() {
            Ok(num) => {
                available_balance = num;
            }
            Err(e) => log_str(&format!("err: {e}")),
        }

        let schedule = Schedule {
            begin: begin,
            last_take: begin,
            end: end,
            available_balance: available_balance,
            taken_balance: 0,
            t: t,
        };

        match self.per_b.get(&b) {
            Some(mut per_ft) => {
                match per_ft.get(&ft) {
                    Some(mut per_a) => {
                        match per_a.get(&a) {
                            Some(schedule) => {
                                // todo add
                            }
                            None => {
                                // create
                                per_a.insert(&a, &schedule);
                            }
                        }
                    }
                    None => {
                        // create
                        let mut per_a: LookupMap<AccountId, Schedule> =
                            LookupMap::new(b"per_a".to_vec());
                        per_a.insert(&a, &schedule);
                        per_ft.insert(&ft, &per_a);
                    }
                }
            }
            None => {
                // create
                let mut per_a: LookupMap<AccountId, Schedule> = LookupMap::new(b"per_a".to_vec());
                per_a.insert(&a, &schedule);

                let mut per_ft: LookupMap<AccountId, LookupMap<AccountId, Schedule>> =
                    LookupMap::new(b"per_ft".to_vec());
                per_ft.insert(&ft, &per_a);

                self.per_b.insert(&b, &per_ft);
            }
        }

        // return PromiseOrValue::Value(U128(0));
        return "0".to_string();
    }

    // pub fn cancel(&mut self, b: AccountId, ft: AccountId) {} // todo

    // clear DEBUG
    pub fn clear(&mut self, b: AccountId) {
        self.per_b.remove(&b);
    }

    // take
    pub fn take(&mut self, a: AccountId, b: AccountId, ft: AccountId) {
        log_str(&format!("a: {a}"));
        log_str(&format!("b: {b}"));
        log_str(&format!("ft: {ft}"));
        match self.per_b.get(&b) {
            Some(per_ft) => {
                match per_ft.get(&ft) {
                    Some(per_a) => {
                        match per_a.get(&a) {
                            Some(schedule) => {
                                log_str(&format!("schedule"));
                                log_str(&schedule.begin.to_string());
                                log_str(&schedule.end.to_string());
                                log_str(&schedule.last_take.to_string());
                                log_str(&schedule.taken_balance.to_string());
                                log_str(&schedule.available_balance.to_string());

                                // math
                                let total_balance =
                                    schedule.available_balance + schedule.taken_balance;
                                log_str(&total_balance.to_string());
                                let elapsed = env::block_timestamp() - schedule.begin;
                                log_str(&elapsed.to_string());
                                let total_time = schedule.end - schedule.begin;
                                log_str(&total_time.to_string());
                                let time_fraction = elapsed as f64 / total_time as f64;
                                log_str(&time_fraction.to_string());
                                let can_be_taken_balance =
                                    (time_fraction * total_balance as f64) as Balance;
                                log_str(&can_be_taken_balance.to_string());
                                let take_amount = can_be_taken_balance - schedule.taken_balance;
                                log_str(&take_amount.to_string());

                                // send
                                let memo = None;
                                ext(ft).with_attached_deposit(1).ft_transfer(b, convert(take_amount), memo);
                            }
                            None => {
                                log_str(&format!("no per_a"));
                                env::panic_str("no per_a")
                            },
                        }
                    }
                    None => {
                        log_str(&format!("no per_ft"));
                        env::panic_str("no per_ft")
                    },
                }
            }
            None => {
                log_str(&format!("no per_b"));
                env::panic_str("no per_b")
            },
        }
    }

    // view
    pub fn view(&self, a: AccountId, b: AccountId, ft: AccountId) -> String {
        match self.per_b.get(&b) {
            Some(per_ft) => match per_ft.get(&ft) {
                Some(per_a) => match per_a.get(&a) {
                    Some(schedule) => {
                        return format!(
                            "{}-{}-{} * {}/{}",
                            schedule.begin,
                            schedule.last_take,
                            schedule.end,
                            schedule.taken_balance,
                            schedule.available_balance
                        );
                    }
                    None => {
                        return "a not found".to_string();
                    }
                },
                None => {
                    return "ft not found".to_string();
                }
            },
            None => {
                return "b not found".to_string();
            }
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = FFT::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             contract.get_greeting(),
//             "Hello".to_string()
//         );
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = FFT::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             contract.get_greeting(),
//             "howdy".to_string()
//         );
//     }
// }
