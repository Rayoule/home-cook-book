use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};
use std::io::BufReader;

use super::auth_utils::LoginAccountCollection;
use crate::app::components::auth::auth_utils::{LoginAccount, UserLoginState};


#[server]
pub async fn try_login(account: LoginAccount) -> Result<bool, ServerFnError> {
    
    // check if username is not empty
    if account.username.is_empty() {
        Err(ServerFnError::ServerError("Username is empty".to_string()))
    }
    // check if password is not empty
    else if account.password.is_empty() {
        Err(ServerFnError::ServerError("Password is empty".to_string()))
    }
    // proceed to auth
    else {
        // check if account is correct
        match check_account(&account).await {
            Ok(account_exists) => {
                if account_exists {
                    log_in_user(&account).await
                } else {
                    return Err(ServerFnError::ServerError("Invalid username and/or password.".to_string()));
                }
            },
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
    }
}


#[cfg(feature = "ssr")]
pub async fn check_account(submission: &LoginAccount) -> Result<bool, ServerFnError> {
    // Get the current working directory
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // Combine the path and filename
    use crate::app::components::auth::auth_utils::ACCOUNTS_FILE_NAME;
    let file_path = current_dir.join(ACCOUNTS_FILE_NAME);
    println!("Attempting to open: {:?}", file_path);

    // Open the file
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON into a `LoginStates` struct
    let accounts: LoginAccountCollection = serde_json::from_reader(reader)?;

    // DEBUG
    log!("{:?}", &accounts);

    // Check if it contains the submitted account
    Ok(accounts.contains_account(submission))
}


#[server]
pub async fn check_login(user: LoginAccount) -> Result<bool, ServerFnError> {
    // TODO
    // A function that will check if the user is logged in or not
    // plus replace the login state if the user log from another ip
    // will redirect to login page if not logged in
    Ok(true)
}

#[cfg(feature = "ssr")]
pub async fn log_in_user(submission: &LoginAccount) -> Result<bool, ServerFnError> {

    log!("Attempt to log in user {:?}", submission.username);

    // Fetch ip address
    use actix_web::dev::ConnectionInfo;
    let cur_ip = leptos_actix::extract::<ConnectionInfo>().await?;
    let cur_ip = cur_ip.peer_addr().unwrap();
    //log!("current ip: {:?}", &cur_ip);

    // Fetch state
    use actix_web::web::Data;
    use ev::submit;
    use crate::app::components::auth::auth_utils::SharedLoginStates;
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    let mut login_states = shared_login_states_lock.clone();

    // check if already logged in
    if login_states.iter().any(|x| {
        x.current_ip == cur_ip
    }) {
        // TODO if already logged in then ignore or refresh ?
        log!("User {:?} already logged in.", submission.username);
        Ok(false)
    } else {
        log!("Logging user {:?} in...", submission.username);
        let new_login_state =
            UserLoginState {
                username:   submission.username.clone(),
                current_ip: cur_ip.to_string(),
                log_date:   SystemTime::now()
            };
            
        login_states.push(new_login_state);

        *shared_login_states_lock = login_states;

        log!("User {:?} is now logged in.", submission.username);
        
        Ok(true)
    }
}
