use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};
use std::io::BufReader;

use super::auth_utils::{LoginAccountCollection, SharedLoginStates};
use crate::app::components::auth::auth_utils::LoginAccount;



#[server]
pub async fn try_login(account: LoginAccount) -> Result<bool, ServerFnError> {
    
    // check if username is not empty
    if account.username.is_empty() {
        return Err(ServerFnError::ServerError("Username is empty".to_string()))
    }

    // check if password is not empty
    else if account.password.is_empty() {
        return Err(ServerFnError::ServerError("Password is empty".to_string()))
    }

    // proceed to auth
    else {
        // check if account is correct
        if check_account(account).await? {
            // Then login
        } else {
            return Err(ServerFnError::ServerError("Invalid username and/or password.".to_string()));
        }

        // test
        

        // log in user
        //log_in_user();
    }

    Ok(true)
}


#[cfg(feature = "ssr")]
pub async fn check_account(submission: LoginAccount) -> Result<bool, ServerFnError> {
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

    // Check if it contains the submitted account
    Ok(accounts.contains_account(&submission))
}


#[cfg(feature = "ssr")]
pub async fn check_login(ip_address: String) -> Result<bool, ServerFnError> {
    // TODO
    // Check if logged in
    use actix_web::web::Data;
    use leptos_actix;
    /*let login_states: Data<LoginStates> = leptos_actix::extract().await?;
    let login_states = *login_states.clone().clone();*/

    //Ok(login_states.check_if_logged_in(ip_address))
    Ok(true)
}

#[cfg(feature = "ssr")]
pub async fn log_in_user(submission: &LoginAccount) -> Result<(), ServerFnError> {

    use actix_web::dev::ConnectionInfo;
    let connexion_info = leptos_actix::extract::<ConnectionInfo>().await?;
    println!("{:?}", connexion_info);

    //let shared_loginf

    Ok(())
}