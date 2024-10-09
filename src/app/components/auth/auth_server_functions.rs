use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{Duration, SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};
use std::io::BufReader;

use super::auth_utils::LoginAccountCollection;
use crate::app::components::auth::auth_utils::{
    LoginAccount, UserLoginState,
};

#[cfg(feature = "ssr")]
use crate::app::components::auth::auth_utils::*;


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
/// This function will run on almost every request to check the login
pub async fn user_login_check(user: LoginAccount) -> Result<bool, ServerFnError> {
    check_login(user).await
}


#[cfg(feature = "ssr")]
pub async fn check_login(user: LoginAccount) -> Result<bool, ServerFnError> {

    // Fetch state
    use actix_web::web::Data;
    use crate::app::components::auth::auth_utils::SharedLoginStates;
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    // Fetch request ip
    let cur_ip = fetch_request_ip(&user).await?;

    log!("A");
    
    // Compare infos from state and from user
    let mut is_logged_in = false;

    use std::ops::DerefMut;
    shared_login_states_lock.deref_mut().retain_mut(|logged_user: &mut UserLoginState| {
        log!("B");
        let current_time = SystemTime::now();
        let login_duration = Duration::from_secs(LOG_PERSISTANCE_DURATION_SECONDS);
        // If IP is logged in
        if logged_user.current_ip == cur_ip {
            log!("C");
            // Check that the log date isn't passed out
            if logged_user.log_date + login_duration > current_time {
                log!("D");
                // If passed out, logout
                false
            } else {
                // Then check the username
                if logged_user.username != user.username {
                    log!("E");
                    // If not matching, logout
                    false
                } else {
                    log!("F");
                    // If the username is matching, then update the log date and validate login
                    logged_user.log_date = current_time;
                    is_logged_in = true;
                    true
                }
            }
        } else {
            log!("...");
            // IP not matching -> not that user
            // We return true so that the entry is not removed
            true
        }
    });

    Ok(is_logged_in)
}

#[cfg(feature = "ssr")]
pub async fn log_in_user(submission: &LoginAccount) -> Result<bool, ServerFnError> {

    log!("Attempt to log in user {:?}", submission.username);

    // Fetch ip address
    let cur_ip = fetch_request_ip(submission).await?;

    // Fetch state
    use actix_web::web::Data;
    use crate::app::components::auth::auth_utils::SharedLoginStates;
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    let mut login_states = shared_login_states_lock.clone();

    log!("Current state is: {:?}", login_states);
    log!("while User is: {:?}, with IP: {:?}", submission, &cur_ip);

    // Compare infos from state and from user
    let mut is_logged_in = false;

    use std::ops::DerefMut;
    shared_login_states_lock.deref_mut().retain_mut(|logged_user: &mut UserLoginState| {
        log!("Checking entry...");
        let current_time = SystemTime::now();
        let login_duration = Duration::from_secs(LOG_PERSISTANCE_DURATION_SECONDS);
        // If IP is logged in
        if logged_user.current_ip == cur_ip {
            // Check that the log date isn't passed out
            if logged_user.log_date + login_duration > current_time {
                // If passed out, logout
                false
            } else {
                // If log not passed out, Then check the username
                if logged_user.username != submission.username {
                    // If not matching, logout
                    false
                } else {
                    // If the username is matching, then update the log date and validate login
                    logged_user.log_date = current_time;
                    is_logged_in = true;
                    true
                }
            }
        } else {
            log!("...");
            // IP not matching -> not that user
            // We return true so that the entry is not removed
            true
        }
    });

    if !is_logged_in {
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

    } else {
        // Return false if the user was already logged in
        log!("User {:?} was already logged in.", submission.username);
        Ok(false)
    }
}