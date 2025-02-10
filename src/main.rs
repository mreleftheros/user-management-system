fn main() {
    let access = user_management_system::login().unwrap_or_else(|| {
        eprintln!("Failed to login to service");
        std::process::exit(1);
    });
}
