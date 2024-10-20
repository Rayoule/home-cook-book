use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "ssr")]
use std::sync::{Mutex, MutexGuard, Arc};


pub const ACCOUNTS_FILE_NAME: &'static str = "hcb_auth.json";
pub const LOG_PERSISTANCE_DURATION_SECONDS: u64 = 7200; // 7200s = 2h;


// Struct found in the JSON auth file along with the .exe
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
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


#[cfg(feature = "ssr")]
pub async fn fetch_request_ip() -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;

    match leptos_actix::extract::<ConnectionInfo>().await {
        Ok(connection_info) => {
            if let Some(current_ip) = connection_info.peer_addr() {
                let fetched_ip = current_ip.to_string();
                log!("IP fetched: {:?}", fetched_ip);
                Ok(fetched_ip)
            } else {
                Err(ServerFnError::ServerError("IP not found in HttpRequest.".to_string()))
            }
        },
        Err(e) => {
            log!("ERROR in IP Fetching: {:?}", e.to_string());
            Err(ServerFnError::ServerError(e.to_string()))
        }
    }
}
