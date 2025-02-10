use sha2::{Digest, Sha256};
use std::io;

pub fn get_input(prompt_message: &str) -> String {
    let mut s = String::new();
    loop {
        println!("{prompt_message}");
        s.clear();
        if io::stdin().read_line(&mut s).is_ok() {
            break;
        }
    }
    s.trim().into()
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    format!("{:X}", result)
}
