use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    username: String,
    password: String,
}

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

#[server]
pub async fn try_login(username: String, password: String) -> Result<bool, ServerFnError> {
    
    // check if username is not empty
    if username.as_str().is_empty() {
        return Err(ServerFnError::ServerError("Username is empty".to_string()))
    }

    // check if password is not empty
    else if password.as_str().is_empty() {
        return Err(ServerFnError::ServerError("Password is empty".to_string()))
    }

    // proceed to auth
    else {
        // check username + password
        let exe_path = env::current_exe().unwrap();
        let directory_path = exe_path.parent().unwrap();
        let file = File::open(directory_path).unwrap();

        let users: Vec<UserData> = serde_json::from_reader(file).unwrap();

        // check if account is correct
        let contains_value = users.iter().any(|x| x.username.as_str() == &username);

        // log in user
        //log_in_user();
    }

    Ok(true)
}


#[cfg(feature = "ssr")]
pub async fn check_login(ip_address: String) -> Result<bool, ServerFnError> {
    // TODO
    // Check if logged in
    use actix_web::web::Data;
    use leptos_actix;
    let login_states: Data<LoginStates> = leptos_actix::extract().await?;
    let login_states = *login_states.clone().clone();

    Ok(login_states.check_if_logged_in(ip_address))
}

#[cfg(feature = "ssr")]
pub async fn log_in_user(username: String, ip_address: String) -> Result<(), ServerFnError> {
    // TODO
    // LOG IN USER

    Ok(())
}