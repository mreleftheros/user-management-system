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
