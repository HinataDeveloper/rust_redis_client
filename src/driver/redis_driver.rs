use r2d2::PooledConnection;
use redis::{Client, ConnectionLike};

use crate::Person;

/// # Errors
fn redis_connection() -> Result<PooledConnection<Client>, String> {
    const SERVER_ADDR: &str = "redis://127.0.0.1/";
    let client_result = redis::Client::open(SERVER_ADDR);
    let client = match client_result {
        Ok(cl) => cl,
        Err(err) => return Err(err.to_string()),
    };

    let pool_result = r2d2::Pool::builder().build(client);
    let pool = match pool_result {
        Ok(pool) => pool,
        Err(err) => return Err(err.to_string()),
    };

    let connection_result = pool.get();
    let connection = match connection_result {
        Ok(cnn) => cnn,
        Err(err) => return Err(err.to_string()),
    };

    Ok(connection)
}

#[must_use]
pub fn is_connection_open() -> bool {
    let rc = redis_connection();
    let Ok(cnn) = rc else {
        return false;
    };
    cnn.is_open()
}

#[must_use]
pub fn insert_or_update_person(key: &str, person: &Person) -> bool {
    let rc = redis_connection();
    let Ok(mut cnn) = rc else {
        return false;
    };

    let true = cnn.is_open() else {
        return false;
    };

    let json_person_result = serde_json::to_string(person);
    let json_person = match json_person_result {
        Ok(jp) => jp,
        Err(_err) => return false,
    };

    let final_result = redis::TypedCommands::set(&mut cnn, key, json_person);
    match final_result {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[must_use]
pub fn remove_person(key: &str) -> bool {
    let rc = redis_connection();
    let Ok(mut cnn) = rc else {
        return false;
    };

    let true = cnn.is_open() else {
        return false;
    };

    let final_result = redis::Commands::del(&mut cnn, key);
    match final_result {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[must_use]
pub fn get_person(person_key: &str) -> Option<Person> {
    let rc = redis_connection();
    let Ok(mut cnn) = rc else {
        return None;
    };

    let true = cnn.is_open() else {
        return None;
    };

    let get_result = redis::TypedCommands::get(&mut cnn, person_key);
    let get_option = match get_result {
        Ok(os) => os,
        Err(_err) => return None,
    };

    let s = get_option?;
    let person_result: Result<Person, serde_json::Error> = serde_json::from_str(&s);

    let person = match person_result {
        Ok(prn) => prn,
        Err(_err) => return None,
    };

    Some(person)
}
