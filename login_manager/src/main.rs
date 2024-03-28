use clap::{Parser, Subcommand};
use authentication::{add_user, change_password, delete_user, get_users, hash_password, save_users};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a new user.
    Add {
        /// The user's login name.
        username: String,
        /// The user's password (plain text).
        password: String,
        /// Optional - mark as an admin.
        admin: Option<bool>,
    },
    /// Delete a user.
    Delete {
        /// The user's login name.
        username: String
    },
    /// Change Password
    ChangePassword {
        /// The user's login name.
        username: String,
        /// The user's password (plain text).
        password: String,
    },
}


fn list_users() {
    println!("{:<20} {:<20}", "Username", "Role");
    println!("{:-<40}", "");
    let users = get_users();

    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20} {:<20?}", user.username, user.role);
        })
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }

        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }

        Some(Commands::Delete { username }) => {
            delete_user(username);
        }

        Some (Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        }
        
        None => {
            println!("Run with --help to see instructions.");
        }
    }
}
