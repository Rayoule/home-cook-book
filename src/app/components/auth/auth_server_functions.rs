use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};

use super::auth_utils::LoginAccountCollection;



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

        log!("YOLOOOOOOOOOOOOOOO");

        // check username + password
        /*let exe_path = env::current_exe()?;
        let directory_path = exe_path.parent().unwrap();
        let file = File::open(directory_path)?;

        let users: LoginAccountCollection = serde_json::from_reader(file)?;*/

        // Get the current working directory
        let current_dir = env::current_dir()?;
        println!("Current directory: {:?}", current_dir);

        // Specify the filename
        let filename = "hcb_auth.json";

        // Combine the path and filename
        let file_path = current_dir.join(filename);
        println!("Attempting to open: {:?}", file_path);

        // Open the file
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        // Deserialize the JSON into a `LoginStates` struct
        let login_states: LoginStates = serde_json::from_reader(reader).unwrap();

        // Print the deserialized data
        println!("{:?}", login_states);

        // check if account is correct
        let contains_value = users.accounts.iter().any(|x| x.username.as_str() == &username);

        // test
        use actix_web::dev::ConnectionInfo;
        let connexion_info = leptos_actix::extract::<ConnectionInfo>().await?;
        println!("{:?}", connexion_info);

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
    /*let login_states: Data<LoginStates> = leptos_actix::extract().await?;
    let login_states = *login_states.clone().clone();*/

    //Ok(login_states.check_if_logged_in(ip_address))
    Ok(true)
}

#[cfg(feature = "ssr")]
pub async fn log_in_user(username: String, ip_address: String) -> Result<(), ServerFnError> {
    // TODO
    // LOG IN USER

    Ok(())
}