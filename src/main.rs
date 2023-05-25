use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
    process,
    str::FromStr,
};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments: {err}");
        process::exit(1);
    });

    let mut todo = Todo::new().unwrap_or_else(|err| {
        eprintln!("Initialisation failed: {err}");
        process::exit(1);
    });

    if config.action == "add" {
        todo.insert(config.item);
        todo.save();
    } else if config.action == "complete" {
        match todo.complete(&config.item) {
            Some(_) => {
                todo.save();
            }
            None => {
                println!("'{}' is not present in the list", config.item)
            }
        }
    }
}

struct Config {
    action: String,
    item: String,
}

impl Config {
    fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let action = match args.next() {
            Some(action) => action,
            None => return Err("Didn't get a action string"),
        };

        let item = match args.next() {
            Some(item) => item,
            None => return Err("Didn't get a item string"),
        };

        Ok(Config { action, item })
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, io::Error> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let map: HashMap<String, bool> = contents
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (String::from(v[0]), bool::from_str(v[1]).unwrap()))
            .collect();

        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn save(self) {
        let mut contents = String::new();

        for (key, value) in self.map {
            let record = format!("{key}\t{value}\n");
            contents.push_str(&record);
        }

        match fs::write("db.txt", contents) {
            Ok(_) => {
                println!("todo saved");
            }
            Err(err) => {
                println!("An error occurred: {err}");
            }
        };
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }
}
