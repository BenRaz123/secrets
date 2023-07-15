use clap::{Parser, Subcommand};
use magic_crypt::*;
use sha256::digest;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Subcommands
}
#[derive(Subcommand)]
enum Subcommands {
    New,
    List,
    Remove
}

macro_rules! get_pass {
    ($name:ident with message $msg:expr) => {
        let $name = requestty::Question::password($msg).message($msg).build();
        let $name = requestty::prompt_one($name).unwrap();
        let $name = $name.as_string().unwrap();
    }
}

macro_rules! get_string {
    ($name:ident with message $msg:expr) => {
        let $name = requestty::Question::input($msg).message($msg).build();
        let $name = requestty::prompt_one($name).unwrap();
        let $name = $name.as_string().unwrap();
    }
}

macro_rules! open_db {
    () => {
        sqlite::open(format!("{}/seecrets.db", std::env::var("HOME").expect("Could not get $HOME"))).expect("Could not open db")
    }
}

fn store_password<'a>(password: &'a str) -> sqlite::Result<()> {
    let db = open_db!();

    db.execute(
        "DROP TABLE IF EXISTS password;
        CREATE TABLE password (digest TEXT);"
    )?;

    db.execute(format!(
        "INSERT INTO password VALUES ('{password}');"
    ))?;

    Ok(())
}

fn get_password() -> sqlite::Result<String> {
    let db = open_db!();
    let mut password = String::new();
    db.iterate(
        "SELECT digest FROM password", |pairs| {
            for &(_k, v) in pairs.iter() {
                password = v.unwrap().into();
            }
            true
        }
    )?;
    Ok(password)
}

fn authenticate() -> String {
    if get_password().is_err() {
        get_pass!(new_pass with message "Please enter a new password");
        get_pass!(rep_pass with message "Please repeat that password");
        if new_pass != rep_pass {
            eprintln!("passwords do not match");
            std::process::exit(1);
        }
        store_password(&digest(new_pass)).unwrap();
        return new_pass.into();
    }
    let db_password = get_password().unwrap();
    get_pass!(user_password with message "Please enter your password");
    if db_password != digest(user_password) {
        eprintln!("Passwords do not match");
        std::process::exit(1);
    }
    user_password.into()
}

fn get_largest_index() -> sqlite::Result<u32> {
    let db = open_db!();
    let mut ids_list = vec![];
    db.iterate(
        "SELECT id FROM secrets", |query| {
            for &(_k, v) in query.iter() {
                ids_list.push(v.expect("Error parsing sql").parse::<u32>().unwrap())
            } true
        }
    )?;
    Ok(ids_list.len() as u32)
}

fn new_secret<'a>(secret: &'a str) -> sqlite::Result<()> {
    let db = open_db!();
    let idx = get_largest_index()?;
    db.execute("CREATE TABLE IF NOT EXISTS secrets (id INTEGER NOT NULL, digest TEXT);")?;
    db.execute(format!("INSERT INTO secrets (id, digest) VALUES ({idx}, '{secret}');"))?;
    Ok(())
}

fn remove_secret_at(index: u64) -> sqlite::Result<()> {
    let db = open_db!();
    db.execute(format!("DELETE FROM secrets WHERE id={index};"))?;
    Ok(())
}

fn get_secrets() -> sqlite::Result<Vec<String>> {
    let db = open_db!();
    let mut res: Vec<String> = Vec::new();
    db.iterate(
        "SELECT digest FROM secrets;", |pairs| {
            for &(_k, v) in pairs.iter() {
                res.push(v.unwrap().into())
            } true
        }
    )?;
    Ok(res)
}

fn unencrypt_vec<'a>(password: &'a str, input: Vec<String>) -> Vec<String> {
    let mc = new_magic_crypt!(password, 256);
    let mut output: Vec<String> = Vec::new();
    input.iter().for_each(|x| output.push(mc.decrypt_base64_to_string(x).unwrap()));
    output
}

fn new<'a>(password: &'a str) {
    get_string!(secret with message "Please give a secret");
    let mc = new_magic_crypt!(password, 256);
    let enc_secret = mc.encrypt_str_to_base64(secret);
    new_secret(&enc_secret).expect("Could not write to database");
}

fn list<'a>(password: &'a str) {
    let mc = new_magic_crypt!(password, 256);
    get_secrets()
        .expect("Could not retrieve list")
        .iter()
        .map(|x| mc.decrypt_base64_to_string(x).expect("Could not decrypt vvalue"))
        .for_each(|x| println!("- {x}"));
}

fn remove<'a>(password: &'a str) {
    let index = requestty::Question::select("secret to remove").message("what secret do you want to remove?").choices(unencrypt_vec(password, get_secrets().unwrap())).build();
    let index = requestty::prompt_one(index).unwrap();
    let index = index.as_list_item().unwrap().index;
    remove_secret_at(index as u64).unwrap();
}

fn main() {
    let db = open_db!();
    let password = authenticate();
    db.execute("CREATE TABLE secrets (id INTEGER, digest TEXT);").unwrap_or(());
    let args = Args::parse();
    match args.command {
        Subcommands::New => new(&password),
        Subcommands::List => list(&password),
        Subcommands::Remove => remove(&password),
    }
}
