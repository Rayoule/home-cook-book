use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};


// Struct found in the JSON auth file along with the .exe
#[derive(Deserialize, Debug)]
pub struct LoginAccount {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginAccountCollection {
    pub accounts: Vec<LoginAccount>,
}


// User login state
#[derive(Clone)]
pub struct UserLoginState {
    username: String,
    current_ip: String,
    log_date: SystemTime,
}

#[derive(Default)]
pub struct LoginStates {
    states: Arc<Mutex<Vec<UserLoginState>>>
}
impl LoginStates {
    pub fn check_if_logged_in(&self, ip_address: String) -> bool {
        let binding = self.states.lock().unwrap();
        let lock = binding.clone();
        lock.iter().any(|x| x.current_ip == ip_address)
    }
}

// Init the login states (see in main())
pub fn init_login_states() -> LoginStates {
    LoginStates::default()
}
