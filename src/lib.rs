// a -> b
// a = benefactor
// b = beneficiary

// Find all our documentation at https://docs.near.org
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
    pub per_beneficiary: LookupMap<AccountId, LookupMap<AccountId, LookupMap<AccountId, Schedule>>>,
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
    pub begin: u64, // timestamp
    pub last_take: u64, // timestamp
    pub end: u64,         // timestamp
    pub balance: Balance, // at beginning
    pub t: ScheduleType,
}

// Define the default, which automatically initializes the contract
impl Default for FFT {
    fn default() -> Self {
        Self {
            per_beneficiary: LookupMap::new(b"accounts".to_vec()),
        }
    }
}

fn str_to_account_id(s: &str) -> AccountId {
    s.parse().expect("Invalid AccountId")
}

// Implement the contract structure
#[near_bindgen]
impl FFT {
    // create or add
    pub fn ft_on_transfer(
        &mut self,
        a: AccountId,
        amount: u128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let mut parts = msg.split(',');

        let b_str = parts.next().unwrap_or_default();
        let b = str_to_account_id(b_str);

        let begin_str = parts.next().unwrap_or_default();
        let begin: u64 = begin_str.parse().expect("begin_str not a valid timestamp");

        let end_str = parts.next().unwrap_or_default();
        let end: u64 = end_str.parse().expect("end_str not a valid timestamp");

        let t_str = parts.next().unwrap_or_default();
        let t = ScheduleType::from_str(t_str).unwrap_or_default();

        let ft = env::predecessor_account_id();

        match self.per_beneficiary.get(&b) {
            Some(per_ft) => {
                match per_ft.get(&ft) {
                    Some(per_a) =>  {
                        match per_a.get(&a) {
                            Some(schedule) => {
                                // add
                            },
                            None => {
                                // create
                            },
                        }
                    },
                    None => {
                        // create
                    },
                }
            }
            None => {
                // create
                let schedule = Schedule {
                    begin: begin,
                    last_take: begin,
                    end: end,
                    balance: amount,
                    t: t,
                };

                let mut per_a: LookupMap<AccountId, Schedule> = LookupMap::new(b"per_a".to_vec());
                per_a.insert(&a, &schedule);

                let mut per_ft: LookupMap<AccountId, LookupMap<AccountId, Schedule>> = LookupMap::new(b"per_ft".to_vec());
                per_ft.insert(&ft, &per_a);

                self.per_beneficiary.insert(&b, &per_ft);
            },
        }

        return PromiseOrValue::Value(U128(0));
    }

    pub fn create(&mut self, beneficiary: AccountId, ft: AccountId) {}
    pub fn cancel(&mut self, beneficiary: AccountId, ft: AccountId) {}
    pub fn add(&mut self, beneficiary: AccountId, ft: AccountId) {}
    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn take(&mut self, benefactor: AccountId, beneficiary: AccountId, ft: AccountId) {
        log_str(&format!("benefactor: {benefactor}"));
        log_str(&format!("beneficiary: {beneficiary}"));
        log_str(&format!("ft: {ft}"));
        match self.per_beneficiary.get(&beneficiary) {
            Some(per_benefactor) => {
                match per_benefactor.get(&benefactor) {
                    Some(per_ft) => {
                        match per_ft.get(&ft) {
                            Some(schedule) => {
                                // todo math + send
                            }
                            None => {
                                log_str(&format!("no per_ft"));
                            }
                        }
                    }
                    None => {
                        log_str(&format!("no per_ft"));
                    }
                }
            }
            None => {
                log_str(&format!("no per_benefactor"));
                // env::panic_str("")
                // env::panic_str(format!("The account {} is not registered", &account_id).as_str())
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
