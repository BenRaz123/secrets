pub mod fileutils {
    use std::io::prelude::*;
    pub fn file_is_empty(path: &str) -> bool {
        let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        let reader = std::io::BufReader::new(&file);
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

    pub fn get_file_path() -> String {
        let home = std::env::var("HOME");
        if home.is_err() {
            eprintln!("error: $HOME is not set!");
            std::process::exit(1);
        }
        format!("{}/secrets.txt", home.unwrap())
    }

    pub fn write_to_file(input: &str) {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(get_file_path())
            .unwrap();
        writeln!(file, "{input}").unwrap();
    }

    pub fn read_from_file() -> Option<Vec<String>> {
        let file = std::fs::OpenOptions::new().read(true).open(get_file_path());

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

    pub fn remove_index_from_file(index: u32) -> Result<(), Box<dyn std::error::Error>> {
        let file_read = std::fs::OpenOptions::new()
            .read(true)
            .open(get_file_path())?;

        let reader = std::io::BufReader::new(&file_read);

        let mut buf: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        buf.remove(index as usize);

        let mut file_write = std::fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(get_file_path())?;

        file_write.write_all(format!("{}\n", buf.join("\n")).as_bytes())?;

        Ok(())
    }
}
