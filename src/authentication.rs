use crate::util;
const MAX_TRIES: u8 = 3;

pub fn try_login() -> bool {
    let mut tries = 0;

    while tries < MAX_TRIES {
        println!("{}/{MAX_TRIES}", tries + 1);
        let username = util::get_input("Enter username:");
        let password = util::get_input("Enter password:");

        if username.to_lowercase() == "admin" && password == "password" {
            println!("Welcome {username}!");
            return true;
        } else {
            tries += 1;
            println!("Wrong credentials");
            continue;
        }
    }
    println!("Too many failed attempts");
    false
}
