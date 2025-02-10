use std::io;

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello George", greet_user("George"));
    }
}
