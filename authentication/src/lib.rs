use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};


pub fn save_users(users: HashMap<String, User>) {
    let path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(path, users_json).unwrap();
}

pub fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin { LoginRole::Admin } else { LoginRole::User };

    let user = User::new(&username, &password, role);
    users.insert(username, user);

    save_users(users);
}

pub fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("User not found");
    }
}

pub fn change_password(username: String, password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = hash_password(&password);
        save_users(users);
    } else {
        println!("User not found");
    }
}

pub fn greet_user(name: &str) -> String {
    return format!("Hello, {}!", name);
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();
    let password = hash_password(password);

    return match users.get(&username) {
        None => None,

        Some(user) => {
            if user.password == password { Some(LoginAction::Granted(user.role.clone())) } else { Some(LoginAction::Denied) }
        }
    };
}

pub fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");

    return buf.trim().to_string();
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        return Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        };
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);

    return format!("{:X}", hasher.finalize());
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("user".to_string(), User::new("user", "password", LoginRole::User));

    return users;
}

pub fn get_users() -> HashMap<String, User> {
    let path = Path::new("users.json");
    if path.exists() {
        let users_json = std::fs::read_to_string(path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();

        return users;
    }

    let users = get_default_users();
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(path, users_json).unwrap();

    return users;
}

pub fn get_users_hashmap() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("user".to_string(), User::new("user", "password", LoginRole::User));

    return users;
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello, John!", greet_user("John"));
    }

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("user", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("user", ""), Some(LoginAction::Denied));
        assert_eq!(login("", "password"), None);
    }
}
