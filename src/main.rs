fn main() {
    let result = user_management_system::try_login();
    if result {
        println!("Logged in");
    } else {
        println!("Failed to login");
    }
}
