use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::collections::{BTreeMap, HashMap};
use std::collections::VecDeque;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::marker::PhantomData;
use std::process;

#[derive(Parser)]
#[grammar = "sql.pest"]
struct SQLParser;

pub struct DBMS {
    error: Result<(), String>,
    databases: HashMap<String, DataBase>,
    curr_db: Option<String>,
}

impl DBMS {
    const PROMPT: &str = " > ";
    pub fn new() -> Self {
        Self {
            error: Ok(()),
            databases: HashMap::new(),
            curr_db: None,
        }
    }
    pub fn interactive(&mut self) {
        let mut line = String::new();
        loop {
            stdout().write(DBMS::PROMPT.as_bytes()).unwrap();
            stdout().flush().unwrap();
            stdin().read_line(&mut line).unwrap();
            line.clear();
        }
    }
    pub fn sql_from_file(&mut self, path: &str) {
        match fs::read_to_string(path) {
            Err(e) => println!("Error reading from {}\n{}", path,  e),
            Ok(k) => {
                match SQLParser::parse(Rule::SQL, &k) {
                    Ok(k) => {
                        for command in k {
                            match self.run(command) {
                                Ok(k) => println!("{k}"),
                                Err(e) => println!("{e}")
                            }
                        }
                    },
                    Err(e) => println!("Error parsing {}\n{}", path, e)
                }
            }
        }
    }
    fn run(&mut self, command: Pair<Rule>) -> Result<String, String>{
        match command.as_rule() {
            Rule::create => {
                let mut it = command.into_inner();
                match it.next().unwrap().as_rule() {
                    Rule::database => {
                        let name = it.next().unwrap().as_str();
                        if self.databases.contains_key(name) {
                            Err(format!("!Failed to create database {} because it already exists.", name))
                        } else {
                            self.databases.insert(String::from(name), DataBase::new());
                            Ok(format!("Database {} created.", name))
                        }
                    },
                    Rule::table => {
                        let name = it.next().unwrap().as_str();
                        match self.curr_db {
                            Some(db) => {
                            },
                            None => Err(format!("!No database supplied"))
                        }
                    },
                    _ => Err(format!("An uknown parsing error happened on line: {}", line!()))
                }
            },
            Rule::drop => {
                let mut it = command.into_inner();
                match it.next().unwrap().as_rule() {
                    Rule::database => {
                        let name = it.next().unwrap().as_str();
                        if self.databases.contains_key(name) {
                            self.databases.remove(&String::from(name));
                            Ok(format!("Database {} deleted.", name))
                        } else {
                            Err(format!("!Failed to delete database {} because it does not exist.", name))
                        }
                    },
                    _ => Err(format!("An uknown parsing error happened on line: {}", line!()))
                }
            },
            Rule::_use => {
                let mut it = command.into_inner();
                let name = it.next().unwrap().as_str();
                if self.databases.contains_key(name) {
                    self.curr_db = Some(String::from(name));
                    Ok(format!("Using database {}.", name))
                } else {
                    Err(format!("!Cannot use database {} as it does not exist", name))
                }
            },
            Rule::exit => {
                process::exit(0);
            }
            _ => Err(format!("Command \"{}\" was parsed but could not be ran", command.as_str()))
        }
    }
    pub fn show_database(&self) {
        for key in &self.databases {
            println!("{}", key.0);
        }
    }
}

struct DataBase {
    tables: HashMap<String, ()>,
}

impl DataBase {
    fn new() -> Self {
        Self {
            tables: HashMap::new()
        }
    }
}

