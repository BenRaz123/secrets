use clap::{Parser, Subcommand};
use requestty::Question;
use std::fs;
use std::process::exit;
use std::io::BufReader;
use std::io::prelude::*;

type Password = String;

#[derive(Parser, Debug)]
#[command(name = "secrets")]
#[command(author = "Ben Raz <ben.raz2008@gmail.com>")]
#[command(about = "a password-protected list of secrets")]
#[command(version = "0.0.1")]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

fn file_is_empty(path: &str) -> bool {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(path).unwrap();
    let reader = BufReader::new(&file);
    let buf: String = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>().join("");
    match &buf[..] {
        "" | " " | "\n" => true,
        _ => false
    }
}

fn get_file_path() -> String {
    let home = std::env::var("HOME");
    if home.is_err() {
        eprintln!("error: $HOME is not set!");
        exit(1);
    }
    format!("{}/secrets.txt", home.unwrap())
}

/// # Hello World
/// _hello_
/// **bye**
fn prompt_password() -> Option<Password> {
    let password = Question::password("password")
        .message("Please enter your password")
        .mask('*')
        .build();
    Some(
        requestty::prompt_one(password)
            .unwrap()
            .as_string()
            .unwrap()
            .into(),
    )
}

/// # Check Password
/// exits with `1` if `pass` is incorrect
fn check_password(pass: &Password) {
    if *pass != String::from("hello") {
        eprintln!("error: Incorrect Password!");
        exit(1);
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
    let file = fs::OpenOptions::new()
        .read(true)
        .open(get_file_path());

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

fn new() {
    let password: Password = prompt_password().unwrap();
    check_password(&password);
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

fn remove_index_from_file(index: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file_read = fs::OpenOptions::new()
        .read(true)
        .open(get_file_path())?;

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

fn remove() {
    check_password(&prompt_password().unwrap());
    if file_is_empty(&get_file_path()) {
        eprintln!("You don't have any secrets, silly!");
        exit(1);
    }
    let lines: Vec<String> = read_from_file().unwrap().into_iter().filter(|l| l != "\n" || l != " " || l != "").collect();
    let user_deletion = Question::select("remove secret")
        .message("Which secret(s) do you want to remove?")
        .choices(&lines)
        .build();
    let user_deletion = requestty::prompt_one(user_deletion).unwrap();
    let user_index = user_deletion.as_list_item().unwrap().index;
    remove_index_from_file(user_index as u32).unwrap();
}

fn list() {
    check_password(&prompt_password().unwrap());
    let lines = read_from_file();
    match &lines {
        Some(lines) => {
            println!("Your Secrets:\n");
            lines.iter().for_each(|line| println!("{line}"));
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
        exit(0);
    }
    let subcommand = args.command.unwrap();
    match subcommand {
        Subcommands::New => new(),
        Subcommands::List => list(),
        Subcommands::Remove => remove(),
    }
}
