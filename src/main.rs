pub mod backend;

use backend::*;

use clap::{Parser, Subcommand};
use requestty::Question;
use std::process::exit;

#[derive(Parser)]
#[command(name = "secrets")]
#[command(author = "Ben Raz <ben.raz2008@gmail.com>")]
#[command(about = "a password-protected list of secrets")]
#[command(version = "0.0.1")]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
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
    insert_secret(&secret).unwrap();
}

fn remove() {
    authenticate();
    let lines: Vec<String> = get_secrets()
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
    remove_at(user_index as u64).unwrap();
}

fn list() {
    authenticate();
    let lines = get_secrets();
    match &lines {
        Ok(lines) => {
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
        Err(_) => {
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
