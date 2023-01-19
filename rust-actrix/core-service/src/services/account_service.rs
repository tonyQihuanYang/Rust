use crate::{
    data_access::accounts::{create_account, get_account_by_credentials},
    establish_connection,
    models::account::Account,
};
use base64::encode;

pub fn login(username: &str, password: &str) -> Result<Account, ()> {
    let pool = &mut establish_connection();
    match get_account_by_credentials(pool, username, password) {
        Some(account_found) => Ok(account_found),
        None => Err(()),
    }
}

pub fn register(username: &str, password: &str) -> Result<Account, ()> {
    let pool = &mut establish_connection();
    match create_account(pool, username, password) {
        Ok(account_found) => Ok(account_found),
        Err(_) => Err({}),
    }
}

// pub async fn login(credentials: LoginCredentials) -> Result<User, ()> {
//     let hashed_password = encode(&credentials.password);
//     match find_user_by_credentials(credentials.username, hashed_password).await {
//         Some(user_found) => Ok(user_found),
//         None => Err(()),
//     }
// }
