use crate::models::account::{Account, NewAccount};
use diesel::prelude::*;

pub fn get_account_by_credentials(
    conn: &mut PgConnection,
    _username: &str,
    _password: &str,
) -> Option<Account> {
    use crate::schema::accounts::dsl::*;

    let result = accounts
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .first::<Account>(conn)
        .optional()
        .expect("Unexpected error gett account by credentials");

    result
}

pub fn get_account_by_id(conn: &mut PgConnection, _id: &i32) -> Option<Account> {
    use crate::schema::accounts::dsl::*;

    let result = accounts
        .filter(id.eq(_id))
        .first::<Account>(conn)
        .optional()
        .expect("Unexpected error gett account by Id");

    result
}

pub fn create_account(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<Account, diesel::result::Error> {
    let new_account = NewAccount { username, password };

    use crate::schema::accounts;
    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
}

pub fn delete_account(conn: &mut PgConnection, _id: &i32) -> usize {
    use crate::schema::accounts::dsl::*;
    diesel::delete(accounts.filter(id.eq(_id)))
        .execute(conn)
        .expect("Error deleting account")
}
