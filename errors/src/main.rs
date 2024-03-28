use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No user found")]
    NotFound,

    #[error("Too many users found")]
    Default,
}

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
struct User {
    user: String,
}

fn load_users() -> Result<Vec<User>, UsersError> {
    let raw_text = read_file("users.json").map_err(|_| UsersError::NotFound)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UsersError::Default)?;

    return Ok(users);
}

fn read_file(s: &str) -> Result<String, std::io::Error> {
    let path = std::path::Path::new(s);

    let content = std::fs::read_to_string(path)?;

    return Ok(content);
}

fn main() {}
