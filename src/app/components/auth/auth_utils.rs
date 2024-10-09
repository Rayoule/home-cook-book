use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};


pub const ACCOUNTS_FILE_NAME: &'static str = "hcb_auth.json";


// Struct found in the JSON auth file along with the .exe
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginAccount {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginAccountCollection {
    pub accounts: Vec<LoginAccount>,
}
impl LoginAccountCollection {
    pub fn contains_account(&self, submission: &LoginAccount) -> bool {
        self.accounts.iter().any(|x| {
            x.username == submission.username
            && x.password == submission.password
        })
    }
}


// User login state
#[derive(Clone, Debug)]
pub struct UserLoginState {
    pub username: String,
    pub current_ip: String,
    pub log_date: SystemTime,
}

#[cfg(feature = "ssr")]
#[derive(Default, Clone)]
pub struct SharedLoginStates {
    pub states: Arc<Mutex<Vec<UserLoginState>>>
}
#[cfg(feature = "ssr")]
impl SharedLoginStates {
    pub fn init_states() -> Self {
        SharedLoginStates {
            states: Arc::new(Mutex::new(vec![]))
        }
    }
}
