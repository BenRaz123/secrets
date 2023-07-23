use clap::{Parser, Subcommand};
use magic_crypt::*;
use sha256::digest;
use rand::seq::SliceRandom;

#[derive(Parser)]
struct Args {
    /// Allows you to reset the password. NOTE: erases your data
    #[arg(short, long="reset-password")]
    reset_password: bool,
    #[command(subcommand)]
    command: Option<Subcommands>
}
#[derive(Subcommand)]
enum Subcommands {
    New,
    List,
    Remove
}

macro_rules! fail {
    ($msg:expr) =>  {
        color_print::cprintln!("<red,bold>error:</red,bold>: {}", $msg);
    }
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

fn new_salt() -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*\\/<>;:".chars().collect();
    let mut rng = rand::thread_rng();
    let mut res = String::new();
    for _ in 0..=20 {
        res.push(*chars.clone().choose(&mut rng).unwrap());  
    }
    res
}

fn store_password<'a>(password: &'a str, salt: &'a str) -> sqlite::Result<()> {
    let db = open_db!();

    db.execute(
        "DROP TABLE IF EXISTS password;
        DROP TABLE IF EXISTS secrets;
        CREATE TABLE password (digest TEXT, salt TEXT);"
    )?;

    db.execute(format!(
        "INSERT INTO password VALUES ('{password}', '{salt}');"
    ))?;

    Ok(())
}

fn get_salt() -> sqlite::Result<String> {
    let db = open_db!();
    let mut salt = String::new();
    db.iterate(
       "SELECT salt FROM password;", |pairs| {
            for &(_k, v) in pairs.iter() {
                salt = v.unwrap().into();
            } true
       } 
    )?;
    Ok(salt)
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
        let salt = new_salt();
        store_password(&digest(new_pass.to_string()+&salt), &salt).unwrap();
        return new_pass.into();
    }
    let db_password = get_password().unwrap();
    let salt = get_salt().expect("Password authentication failed: Could not get salt!");
    get_pass!(user_password with message "Please enter your password");
    if db_password != digest(user_password.to_string()+&salt) {
        eprintln!("Passwords do not match");
        std::process::exit(1)
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
        .for_each(|x| color_print::cprintln!("<green>-</> <bold,blue>{x}</>"));
}

fn remove<'a>(password: &'a str) {
    let indexes = requestty::Question::multi_select("secret to remove").message("what secret do you want to remove?").choices(unencrypt_vec(password, get_secrets().unwrap())).build();
    let indexes = requestty::prompt_one(indexes).unwrap();
    let indexes = indexes.as_list_items().unwrap();
    indexes.iter().for_each(|x| remove_secret_at(x.index as u64).unwrap());
}

fn prompt_new_pass() {
    get_pass!(new_pass with message "Please give a new password");
    get_pass!(rep_pass with message "Please repeat that password");
    if new_pass != rep_pass {
        fail!("Passwords do not match!");
    }
    let salt = new_salt();
    store_password(&digest(new_pass.to_owned()+&salt), &salt).unwrap();
}

fn parse_args() -> Option<Subcommands> {
    let args = Args::parse();
    let reset_pass = args.reset_password;
    if reset_pass {
        prompt_new_pass(); 
        std::process::exit(0);
    }
    match args.command {
        Some(subcommand) => Some(subcommand),
        None => {fail!("Expected `new`, `list`, or `remove`"); None},
    }

}

fn main() {
    let db = open_db!();
    db.execute("CREATE TABLE secrets (id INTEGER, digest TEXT);").unwrap_or(());
    let args = parse_args();
    let password = authenticate();
    match args.unwrap() {
        Subcommands::New => new(&password),
        Subcommands::List => list(&password),
        Subcommands::Remove => remove(&password),
    }
}
