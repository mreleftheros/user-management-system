use crate::util;
const MAX_ATTEMPTS: u8 = 3;

#[derive(Debug)]
pub enum Access {
    Granted(Role),
    Denied,
}

#[derive(Debug, Clone)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone)]
struct UserDao {
    username: String,
    password: String,
    role: Role,
}

impl UserDao {
    fn new(username: &str, password: &str, role: Role) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
            role,
        }
    }
    fn get_all() -> Vec<UserDao> {
        vec![
            Self::new("admin", "password", Role::Admin),
            Self::new("bob", "password", Role::User),
        ]
    }
}

pub fn login() -> Option<Access> {
    let mut attempts = 0;
    let user_daos = UserDao::get_all();

    while attempts < MAX_ATTEMPTS {
        println!("{}/{MAX_ATTEMPTS}", attempts + 1);
        let username = util::get_input("Enter username:");
        let username = username.to_lowercase();
        let password = util::get_input("Enter password:");

        let user = user_daos
            .iter()
            .find(|u| u.username.to_lowercase() == username && u.password == password)
            .cloned();
        let Some(user) = user else {
            attempts += 1;
            println!("Wrong credentials");
            continue;
        };

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
