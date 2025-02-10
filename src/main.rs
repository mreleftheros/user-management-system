use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all users
    List,
}

fn print_users() {
    let all_users = user_management_system::User::get_all();
    println!("{:<20} {:<20}", "Username", "Role");
    println!("{:-<40}", "");
    all_users
        .into_iter()
        .for_each(|(_, v)| println!("{:<20} {:<20}", v.username, v.role));
}

fn main() {
    // let access = user_management_system::login().unwrap_or_else(|| {
    //     eprintln!("Failed to login to service");
    //     std::process::exit(1);
    // });
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        eprintln!("Run with --help to see instructions");
        std::process::exit(1);
    };
    match command {
        Command::List => {
            print_users();
        }
    }
}
