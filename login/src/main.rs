use authentication::{greet_user, login, LoginAction, LoginRole, read_line};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter username:");
        let username = read_line();

        println!("Enter password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("{}", greet_user("Admin"));
                break;
            }
            
            Some(LoginAction::Granted(LoginRole::User)) => {
                println!("{}", greet_user("User"));
                break;
            }

            Some(LoginAction::Denied) => {
                println!("Incorrect username or password. Try again.");
                tries += 1;
                if tries == 3 {
                    println!("Too many tries. Exiting.");
                    break;
                }
            }

            None => {
                println!("new user");
            }
        }
    }
}
