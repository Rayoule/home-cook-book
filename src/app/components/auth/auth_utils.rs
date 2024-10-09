use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::{SystemTime, UNIX_EPOCH}};
use std::sync::{Arc, Mutex};


pub const ACCOUNTS_FILE_NAME: &'static str = "hcb_auth.json";


// Struct found in the JSON auth file along with the .exe
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    username: String,
    current_ip: String,
    log_date: SystemTime,
}

#[derive(Default)]
pub struct SharedLoginStates {
    states: Arc<Mutex<Vec<UserLoginState>>>
}
impl SharedLoginStates {

    pub fn get_inner_login_states(self) -> Vec<UserLoginState> {
        self.states.lock().unwrap().clone()
    }

    pub fn check_if_logged_in(&self, ip_address: String) -> bool {
        let binding = self.states.lock().unwrap();
        let lock = binding.clone();
        lock.iter().any(|x| x.current_ip == ip_address)
    }

    #[cfg(feature = "ssr")]
    pub async fn add_login(self, username: String) -> Result<bool, ServerFnError> {
        let shared_logins = self.states.lock();
        if let Ok(mut shared_logins) = shared_logins {

            let already_logged_in = shared_logins.iter().any(|x| x.username == username);

            if !already_logged_in {
                use actix_web::dev::ConnectionInfo;
                let connexion_info: ConnectionInfo = leptos_actix::extract::<ConnectionInfo>().await?;
                println!("{:?}", connexion_info);
                let ip = connexion_info.peer_addr().unwrap().to_string();
                let new_login_state =
                    UserLoginState {
                        username:   username,
                        current_ip: ip,
                        log_date:   SystemTime::now()
                    };
                shared_logins.push(new_login_state);
                Ok(true)
            } else {
                log!("User Already Logged In");
                Ok(false)
            }
        } else {
            Err(ServerFnError::ServerError("Could not read shared logins state.".to_string()))
        }
    }
}

// Init the login states (see in main())
pub fn init_login_states() -> SharedLoginStates {
    SharedLoginStates::default()
}
