
use std::io::prelude::*;

macro_rules! prompt_pass {
    ($name:ident with message $message:expr) => {
        let question = requestty::Question::password("$name")
            .message($message.to_string())
            .mask('*')
            .build();
        let answer = requestty::prompt_one(question).unwrap();
        let $name = answer.as_string().unwrap().to_string();
    };
}

fn pass_is_empty() -> bool {
    let pass_file = std::fs::OpenOptions::new().read(true).open("./pass.txt");
    if pass_file.is_err() {
        return true;
    }
    let mut pass = String::new();
    pass_file.unwrap().read_to_string(&mut pass).unwrap();
    if pass.trim().is_empty() {
        return true;
    }
    false
}

fn new_password() -> Result<(), Box<dyn std::error::Error>> {
    prompt_pass!(new_pass with message "Please enter a new password");
    prompt_pass!(rep_pass with message "Please repeat that password");

    if rep_pass != new_pass {
        println!("The two passwords do not match!");
        std::process::exit(1);
    }

    let mut pass_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./pass.txt")?;

    pass_file.write_all(new_pass.as_bytes())?;

    Ok(())
}

fn check_password(pass: &str) {
    let mut pass_file = std::fs::OpenOptions::new()
        .read(true)
        .open("./pass.txt")
        .expect("Password file not found");
    let mut official_pass = String::new();
    pass_file
        .read_to_string(&mut official_pass)
        .expect("Could not read password file");
    if pass != official_pass.trim() {
        eprintln!("Incorrect password!");
        std::process::exit(1)
    }
}

fn prompt_password() -> String {
    prompt_pass!(pass with message "Please enter your password");
    pass
}

pub fn authenticate() {
    if pass_is_empty() {
        new_password().unwrap();
        return;
    }
    check_password(&prompt_password());
}
