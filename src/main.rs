use clap::{Parser, Subcommand};
use requestty::Question;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(name = "secrets")]
#[command(author = "Ben Raz <ben.raz2008@gmail.com>")]
#[command(about = "a password-protected list of secrets")]
#[command(version = "0.0.1")]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

macro_rules! prompt_pass {
    ($name:ident with message $message:expr) => {
        let question = Question::password("$name")
            .message($message.to_string())
            .mask('*')
            .build();
        let answer = requestty::prompt_one(question).unwrap();
        let $name = answer.as_string().unwrap().to_string();
    };
}

fn file_is_empty(path: &str) -> bool {
    let file = fs::OpenOptions::new().read(true).open(path).unwrap();
    let reader = BufReader::new(&file);
    let buf: String = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("");
    match &buf[..] {
        "" | " " | "\n" => true,
        _ => false,
    }
}

fn pass_is_empty() -> bool {
    let pass_file = fs::OpenOptions::new().read(true).open("./pass.txt");
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

fn get_file_path() -> String {
    let home = std::env::var("HOME");
    if home.is_err() {
        eprintln!("error: $HOME is not set!");
        exit(1);
    }
    format!("{}/secrets.txt", home.unwrap())
}

fn authenticate() {
    if pass_is_empty() {
        new_password().unwrap();
        return;
    }
    check_password(&prompt_password());
}

fn new_password() -> Result<(), Box<dyn std::error::Error>> {
    prompt_pass!(new_pass with message "Please enter a new password");
    prompt_pass!(rep_pass with message "Please repeat that password");

    if rep_pass != new_pass {
        println!("The two passwords do not match!");
        exit(1);
    }

    let mut pass_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./pass.txt")?;

    pass_file.write_all(new_pass.as_bytes())?;

    Ok(())
}

fn prompt_password() -> String {
    prompt_pass!(pass with message "Please enter your password");
    pass
}

fn check_password(pass: &str) {
    let mut pass_file = fs::OpenOptions::new()
        .read(true)
        .open("./pass.txt")
        .expect("Password file not found");
    let mut official_pass = String::new();
    pass_file
        .read_to_string(&mut official_pass)
        .expect("Could not read password file");
    if pass != official_pass.trim() {
        eprintln!("Incorrect password!");
        exit(1)
    }
}

fn write_to_file(input: &str) {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(get_file_path())
        .unwrap();
    writeln!(file, "{input}").unwrap();
}

fn read_from_file() -> Option<Vec<String>> {
    let file = fs::OpenOptions::new().read(true).open(get_file_path());

    if file.is_err() {
        return None;
    }

    let mut buff = String::new();

    file.unwrap().read_to_string(&mut buff).unwrap();

    let output: Vec<&str> = buff.split("\n").collect::<Vec<&str>>();

    let mut owned_output: Vec<String> = Vec::new();

    output.iter().for_each(|x| owned_output.push(x.to_string()));

    Some(owned_output)
}

fn remove_index_from_file(index: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file_read = fs::OpenOptions::new().read(true).open(get_file_path())?;

    let reader = BufReader::new(&file_read);

    let mut buf: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    buf.remove(index as usize);

    let mut file_write = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(get_file_path())?;

    file_write.write_all(format!("{}\n", buf.join("\n")).as_bytes())?;

    Ok(())
}

fn new() {
    authenticate();
    let secret = Question::input("secret")
        .message("What is your secret?")
        .build();
    let secret: String = requestty::prompt_one(secret)
        .unwrap()
        .as_string()
        .unwrap()
        .into();
    write_to_file(&secret);
}

fn remove() {
    authenticate();
    if file_is_empty(&get_file_path()) {
        eprintln!("You don't have any secrets, silly!");
        exit(1);
    }
    let lines: Vec<String> = read_from_file()
        .unwrap()
        .into_iter()
        .filter(|l| l != "\n" || l != " " || l != "")
        .collect();
    let user_deletion = Question::select("remove secret")
        .message("Which secret(s) do you want to remove?")
        .choices(&lines)
        .build();
    let user_deletion = requestty::prompt_one(user_deletion).unwrap();
    let user_index = user_deletion.as_list_item().unwrap().index;
    remove_index_from_file(user_index as u32).unwrap();
}

fn list() {
    authenticate();
    let lines = read_from_file();
    match &lines {
        Some(lines) => {
            println!("Your Secrets:\n");
            lines
                .iter()
                .filter(|line| {
                    *line != &"".to_string()
                        || *line != &" ".to_string()
                        || *line != &"\n".to_string()
                })
                .for_each(|line| println!("{line}"));
        }
        None => {
            eprintln!("error: File cannot be read");
            exit(1);
        }
    }
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    New,
    List,
    Remove,
}

fn main() {
    let args = Args::parse();

    if args.command.is_none() {
        exit(1);
    }

    let subcommand = args.command.unwrap();

    match subcommand {
        Subcommands::New => new(),
        Subcommands::List => list(),
        Subcommands::Remove => remove(),
    }
}
