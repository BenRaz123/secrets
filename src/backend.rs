use sqlite::*;
use requestty::*;

macro_rules! get_pass {
    ($name:ident with message $msg:expr) => {
        let question = Question::password($msg)
            .message($msg)
            .build();
        let answer = prompt_one(question).unwrap();
        let $name = answer.as_string().unwrap();
    }
}

fn print_failure<'a>(message: &'a str) {
    eprintln!("error: {message}");
    std::process::exit(1);
}

fn get_db_path() -> String {
    format!("{}/secrets.db", std::env::var("HOME").expect("Could not get $HOME!"))
}

pub fn authenticate() {
    scaffold_db().unwrap();
    let password = get_password();
    if password.unwrap().len() < 1 { 
        new_password().unwrap();
        return;
    }
    let password = get_password();
    get_pass!(user_pass with message "Please enter your password");
    if &password.unwrap() != &user_pass.to_string() {
        print_failure("Password does not match!");
    }
}

pub fn scaffold_db() -> sqlite::Result<()> {
    let db = open(get_db_path())?;
    let query = "
        CREATE TABLE IF NOT EXISTS secrets (id INTEGER NOT NULL, secrets TEXT);
    ";
    db.execute(query)?;
    Ok(())
}

pub fn secrets_is_empty() -> bool {
    get_secrets().unwrap().len() > 0
}

pub fn insert_secret<'a>(secret: &'a str) -> sqlite::Result<()> {
    dbg!(get_id().unwrap());
    let id =get_id().unwrap();
    let db = open(get_db_path())?;
    db.execute(format!("INSERT INTO secrets VALUES ({id}, '{secret}');"))?;
    Ok(())
}

fn get_id() -> sqlite::Result<u64> {
    let db = open(get_db_path())?;
    let query = "SELECT id FROM secrets";
    let mut ids: Vec<u64> = Vec::new();
    db.iterate(
        query, |pairs| {
            for &(_, value) in pairs.iter() {
                ids.push(value.unwrap().parse::<u64>().unwrap());    
            }
            true
        }
    )?;

    Ok(*ids.last().unwrap_or(&0u64)+1)
}

pub fn get_secrets() -> sqlite::Result<Vec<String>> {
    let db = open(get_db_path())?;
    let mut res: Vec<String> = Vec::new();
    db.iterate("SELECT secrets FROM secrets", |pairs| {
        for &(_, value) in pairs.iter() {
            res.push(value.unwrap().into());
        }
        true
    })?;
    Ok(res)
}

pub fn new_password() -> sqlite::Result<()> {
    get_pass!(new_pass with message "Please enter a new password");
    get_pass!(rep_pass with message "Please repeat that password");

    if new_pass != rep_pass {
        print_failure("Passwords do not match!");
    }

    let db = open(get_db_path())?;
    db.execute(format!(
        "DROP TABLE IF EXISTS password;
        CREATE TABLE password (password TEXT);
        INSERT INTO password VALUES ('{new_pass}');",
    ))?;
    Ok(())
}

pub fn get_password() -> sqlite::Result<String> {
    let db = open(get_db_path())?;
    db.execute("CREATE TABLE IF NOT EXISTS password (password TEXT)")?;
    let mut res: String = String::new();

    db.iterate("SELECT * FROM password", |pairs| {
        for &(_, value) in pairs.iter() {
            res += value.unwrap();
        }
        true
    })?;

    Ok(res)
}

pub fn remove_at(index: u64) -> sqlite::Result<()> {
    let db = open(get_db_path())?;
    let true_index = index + 1;
    db.execute(format!(
        "DELETE FROM secrets WHERE id = {true_index};"
    ))?;
    Ok(())
}
