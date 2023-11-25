//in retrospect it would have been a lot better to write this program
//using a class and object oriented principles, but for some reason
//I felt like it would be more interesting to write it without
//using a class or other way of declaring class-wide variables.
//This is why you see users getting passed around to every function,
//as it was the easiest way to get all the data around.
//If I didn't run out of time I wanted to make it pass users as a 
//pointer not as a copy, which would have been more "rust-like".

// we use a simple struct to contain users data.
struct User {
    user_name: String,
    password: String,
}


fn main(){
    let mut users = Vec::new();
    mainmenu(users);
}

// Creates the "main menu."
fn mainmenu(mut users: Vec<User>) {
    // this function is returned to after any action
    println!("Welcome to MerchantSoft Launcher.");
    // Code to debug users.
    // println!("Existing Users");
    // for i in &users {
    //     println!("{}",i.user_name);
    // }
    println!("If you'd like to log in, enter Y");
    println!("If you'd like to register a new user, enter X");
    let mut chosen_option: String = String::new();
    std::io::stdin()
        .read_line(&mut chosen_option)
        .expect("Failed to read line");
    
    if chosen_option.trim() == "X" {
        add_user(users);
    } else if chosen_option.trim() == "Y" {
        login_user(users);
    } else{
        println!("Invalid Prompt");
        mainmenu(users);
    }
}

//Simple login system using line reading.
fn login_user(mut users: Vec<User>) {
    let mut user_name: String = String::new();
    let mut password = String::new();
    if users.len() == 0 {
        println!("No users registered!");
        mainmenu(users);
        return
    }
    println!("Enter Username");
    std::io::stdin()
    .read_line(&mut user_name)
    .expect("Failed to read line");
    for i in &users {
        if i.user_name == user_name {
            println!("Found User!");
            println!("Enter Password");
            std::io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
            if i.password == password {
                println!("Welcome {}" , i.user_name);
                // We have successfully logged in! For the purposes of this assignment it takes you back to the beginning
                break;
            }
            // return if any of the data is wrong
            else {
                println!("Invalid Password");
                break;
            }
        } else {
            println!("User Unknown");
            break;
        }
    }
    mainmenu(users);
}

// Adds users to the user list.
fn add_user(mut users: Vec<User>) {
    let mut new_user = User {
        user_name: String::new(),
        password: String::new(),
    };
    // Get the username
    println!("Enter New User Username");
    std::io::stdin()
        .read_line(&mut new_user.user_name)
        .expect("Failed to read line");
    // Get the password
    println!("Enter New User Password");
    std::io::stdin()
        .read_line(&mut new_user.password)
        .expect("Failed to read line");
    // add new user to the users
    users.push(new_user);
    mainmenu(users);
}
