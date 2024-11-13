use leptos::*;
use crate::app::components::auth::auth_utils::*;

#[cfg(feature = "ssr")]
use {
    leptos::logging::*,
    std::{env, fs::File, time::{Duration, SystemTime}},
    std::io::BufReader,
    super::auth_utils::LoginAccountCollection,
    //crate::app::components::auth::auth_utils::*,
    actix_web::web::Data,
    std::sync::MutexGuard,
    crate::app::components::auth::auth_utils::SharedLoginStates
};


#[server]
/// This function is called when a user sent a login request
pub async fn server_try_login(account: LoginAccount) -> Result<bool, ServerFnError> {
    
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
        match check_account_credentials(&account).await {
            Ok(account_exists) => {
                if account_exists {
                    let result: bool = log_in_user(&account).await?;
                    Ok(result)
                } else {
                    return Err(ServerFnError::ServerError("Invalid username and/or password.".to_string()));
                }
            },
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
    }
}


#[server]
/// This function will run on almost every request to check the login
pub async fn server_login_check() -> Result<bool, ServerFnError> {
    let result = check_login().await?;
    Ok(result)
}

#[server]
/// This function will run on almost every request to check the login
pub async fn server_logout() -> Result<(), ServerFnError> {
    log_out_user().await
}


#[cfg(feature = "ssr")]
pub async fn check_account_credentials(submission: &LoginAccount) -> Result<bool, ServerFnError> {
    // Get the current working directory
    let current_dir = env::current_dir()?;

    // Combine the path and filename
    let file_path = current_dir.join(ACCOUNTS_FILE_NAME);

    // Open the file
    let file = match File::open(&file_path) {
        Ok(ok) => ok,
        Err(e) => {
            let custom_error = "Could not fetch the credential file which MUST be located at the project root. Please read README_setup.txt".to_string();
            return Err(ServerFnError::ServerError(custom_error + " " + &e.to_string()));
        }
    };
    let reader = BufReader::new(file);

    // Deserialize the JSON into a `LoginStates` struct
    let accounts: LoginAccountCollection = serde_json::from_reader(reader)?;

    // Check if it contains the submitted account
    Ok(accounts.contains_account(submission))
}



#[cfg(feature = "ssr")]
pub async fn check_login() -> Result<bool, ServerFnError> {

    // Fetch state
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    // Fetch request ip
    let cur_ip = fetch_request_ip().await?;

    // Get Time
    let current_time = SystemTime::now();


    // Clear passed out logins
    shared_login_states_lock.retain_mut(|logged_user: &mut UserLoginState| {
        let login_duration = Duration::from_secs(LOG_PERSISTANCE_DURATION_SECONDS);
        // If log is passed out, then remove login
        if logged_user.log_date + login_duration < current_time {
            false
        } else {
            true
        }
    });
    

    // Compare infos from state and from user
    let mut is_logged_in = false;
    use std::ops::DerefMut;
    shared_login_states_lock.deref_mut().retain_mut(|logged_user: &mut UserLoginState| {
        // If IP is logged in
        if logged_user.current_ip == cur_ip {
            is_logged_in = true;
            logged_user.log_date = current_time;
            true
        } else {
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

    // Fetch state
    use actix_web::web::Data;
    use crate::app::components::auth::auth_utils::SharedLoginStates;
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    // Fetch ip address
    let cur_ip = fetch_request_ip().await?;

    let mut login_states = shared_login_states_lock.clone();

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

#[cfg(feature = "ssr")]
pub async fn log_out_user() -> Result<(), ServerFnError> {

    log!("Attempt to log out user");

    // Fetch state
    use crate::app::components::auth::auth_utils::SharedLoginStates;
    let shared_login_states: Data<SharedLoginStates> = leptos_actix::extract::<Data<SharedLoginStates>>().await?;
    // Get the ownership of the Arc<> inside SharedLoginStates.states
    let shared_login_states = shared_login_states.get_ref().states.clone();
    // Get the MutexGuard to mutate state
    let mut shared_login_states_lock = shared_login_states.lock()?;

    // Fetch ip address
    let cur_ip = fetch_request_ip().await?;

    let mut login_states = shared_login_states_lock.clone();

    // remove the entry matching the ip
    login_states.retain(|entry| entry.current_ip != cur_ip);

    *shared_login_states_lock = login_states;

    log!("User is now logged out.");
    
    Ok(())

}



#[cfg(feature = "ssr")]
pub async fn clear_passed_out_logins(
    mut shared_login_states_lock: MutexGuard<'_, Vec<UserLoginState>>
) -> Result<(), ServerFnError> {
    // Get Time
    let current_time = SystemTime::now();

    use std::ops::DerefMut;
    shared_login_states_lock.deref_mut().retain_mut(|logged_user: &mut UserLoginState| {
        let login_duration = Duration::from_secs(LOG_PERSISTANCE_DURATION_SECONDS);
        // If log is passed out, then remove login
        if logged_user.log_date + login_duration < current_time {
            false
        } else {
            true
        }
    });

    Ok(())
}
