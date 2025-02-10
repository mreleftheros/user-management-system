use serde::{Deserialize, Serialize};
use sha2::digest::HashMarker;
use std::{collections::HashMap, fs, path};

use crate::util::{self, hash_password};
const MAX_ATTEMPTS: u8 = 3;
const USER_JSON_PATH: &str = "users.json";

#[derive(Debug)]
pub enum Access {
    Granted(Role),
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Admin => "admin",
            Self::User => "user",
        };
        write!(f, "{v}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDao {
    username: String,
    password: String,
    role: Role,
}

impl UserDao {
    fn new(username: &str, password: &str, role: Role) -> Self {
        Self {
            username: username.to_owned(),
            password: hash_password(password),
            role,
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub role: Role,
}

pub fn get_all_users() -> HashMap<String, User> {
    let user_daos = get_all_user_daos();
    let mut users = HashMap::new();

    user_daos.into_iter().for_each(|(k, v)| {
        users.insert(k, User::from(v));
    });
    users
}

pub fn set_user(username: String, password: String, admin: bool) {
    let password = hash_password(password.as_str());
    let mut users = get_all_user_daos();

    let role = match admin {
        true => Role::Admin,
        false => Role::User,
    };
    let ud = UserDao::new(username.as_str(), password.as_str(), role);
    users.insert(username, ud);
    save_users(&users);

    println!("New user added");
}

impl From<UserDao> for User {
    fn from(value: UserDao) -> Self {
        Self {
            username: value.username,
            role: value.role,
        }
    }
}

fn get_default_user_daos() -> HashMap<String, UserDao> {
    let mut user_daos = HashMap::new();
    user_daos.insert(
        "admin".to_owned(),
        UserDao::new("admin", "password", Role::Admin),
    );
    user_daos.insert(
        "bob".to_owned(),
        UserDao::new("bob", "password", Role::User),
    );
    user_daos
}

fn save_users(users: &HashMap<String, UserDao>) {
    let file_path = path::Path::new(USER_JSON_PATH);
    let json = serde_json::to_string(users).expect("Failed to turn to json");
    fs::write(file_path, json).expect("Failed to write to path");
}

fn get_all_user_daos() -> HashMap<String, UserDao> {
    let file_path = path::Path::new(USER_JSON_PATH);
    if file_path.exists() {
        // read from file
        let r = fs::read_to_string(file_path).expect("Failed to read path");
        let user_daos = serde_json::from_str(&r).expect("Failed to turn from json");
        user_daos
    } else {
        let user_daos = get_default_user_daos();
        // save to json and into a file before returning them
        save_users(&user_daos);
        user_daos
    }
}

pub fn login() -> Option<Access> {
    let mut attempts = 0;
    let user_daos = get_all_user_daos();

    while attempts < MAX_ATTEMPTS {
        println!("{}/{MAX_ATTEMPTS}", attempts + 1);
        let username = util::get_input("Enter username:");
        let username = username.to_lowercase();
        let password = util::get_input("Enter password:");
        let password = util::hash_password(password.as_str());

        let Some(user) = user_daos.get(&username) else {
            attempts += 1;
            println!("Wrong credentials");
            continue;
        };
        if user.password != password {
            attempts += 1;
            println!("Wrong credentials");
            continue;
        }

        return match user {
            UserDao {
                role: Role::Admin, ..
            } => {
                println!("Welcome admin");
                Some(Access::Granted(Role::Admin))
            }
            UserDao {
                username,
                role: Role::User,
                ..
            } => {
                println!("Welcome {username}");
                Some(Access::Granted(Role::User))
            }
        };
    }
    println!("Too many failed attempts");
    None
}
