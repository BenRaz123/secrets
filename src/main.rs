pub mod fileutils;
pub mod passutils;

use fileutils::fileutils::*;
use passutils::passutils::authenticate;

use clap::{Parser, Subcommand};
use requestty::Question;
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
