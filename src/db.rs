use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[grammar = "sql.pest"]
struct SQLParser;

#[derive(Serialize, Deserialize)]
pub struct DBMS {
    databases: HashMap<String, DataBase>,
    curr_db: Option<String>,
    path: Option<String>,
}

impl DBMS {
    const PROMPT: &str = " > ";
    pub fn new(path: Option<String>) -> Self {
        match path {
            None => {
                Self {
                    databases: HashMap::new(),
                    curr_db: None,
                    path,
                }
            }
            Some(unwrapped_path) => {
                match std::fs::read_to_string(unwrapped_path.as_str()) {
                    Ok(file) => serde_json::from_str(file.as_str()).unwrap(),
                    Err(_) => {
                        Self {
                            databases: HashMap::new(),
                            curr_db: None,
                            path: Some(unwrapped_path)
                        }
                    }
                }
            }
        }
    }
    pub fn interactive(&mut self) {
        let mut line = String::new();
        loop {
            stdout().write(DBMS::PROMPT.as_bytes()).unwrap();
            stdout().flush().unwrap();
            stdin().read_line(&mut line).unwrap();
            match SQLParser::parse(Rule::SQL, &line) {
                Ok(k) => {
                    for command in k {
                        match self.run(command) {
                            Ok(k) => {
                                match k {
                                    Some(s) => println!("{}", s),
                                    None => ()
                                }
                            }
                            Err(e) => println!("{e}")
                        }
                    }
                },
                Err(e) => println!("Error parsing\n{}", e)
            }
            self.save();
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
                                Ok(k) => {
                                    match k {
                                        Some(s) => {
                                            println!("{}", s);
                                            self.save();
                                        },
                                        None => ()
                                    }
                                }
                                Err(e) => println!("{e}")
                            }
                        }
                    },
                    Err(e) => println!("Error parsing {}\n{}", path, e)
                }
            }
        }
    }
    fn run(&mut self, command: Pair<Rule>) -> Result<Option<String>, String>{
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
                            Ok(Some(format!("Database {} created.", name)))
                        }
                    },
                    Rule::table => {
                        match &self.curr_db {
                            Some(db) => {
                                match self.databases.get_mut(db) {
                                    None => Err(format!("!Database {} was deleted", db)),
                                    Some(db) => {
                                        db.create(it)
                                    }
                                }
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
                            Ok(Some(format!("Database {} deleted.", name)))
                        } else {
                            Err(format!("!Failed to delete database {} because it does not exist.", name))
                        }
                    },
                    Rule::table => {
                        match &self.curr_db {
                            Some(db) => {
                                match self.databases.get_mut(db.as_str()) {
                                    Some(db) => db.drop(it),
                                    None => Err(format!("!Database {} was deleted", db.as_str()))
                                }
                            },
                            None => Err(format!("!No database supplied."))
                        }
                    }
                    _ => Err(format!("An uknown parsing error happened on line: {}", line!()))
                }
            },
            Rule::_use => {
                let mut it = command.into_inner();
                let name = it.next().unwrap().as_str();
                if self.databases.contains_key(name) {
                    self.curr_db = Some(String::from(name));
                    Ok(Some(format!("Using database {}.", name)))
                } else {
                    Err(format!("!Cannot use database {} as it does not exist", name))
                }
            },
            Rule::exit => {
                process::exit(0);
            },
            Rule::semicolon => {
                Ok(None)  
            },
            Rule::select => {
                match &self.curr_db {
                    Some(db) => {
                        if self.databases.contains_key(db.as_str()) {
                            match self.databases.get(db).unwrap().select(command.into_inner()) {
                                Ok(k) => Ok(Some(k.unwrap())),
                                Err(e) => Err(e)
                            }
                        } else {
                            Err(format!("!Database {} was deleted", db.as_str()))
                        }
                    },
                    None => Err(format!("!No database supplied"))
                }
            },
            Rule::alter => {
                let mut it = command.into_inner();
                match it.next().unwrap().as_rule() {
                    Rule::table => {
                        match &self.curr_db {
                            Some(db) => {
                                match self.databases.get_mut(db) {
                                    Some(db) => {
                                        db.alter(it)
                                    },
                                    None => Err(format!("!Database {} was deleted.", db.as_str()))
                                }
                            },
                            None => Err(format!("!No database supplied."))
                        }
                    },
                    _ => Err(format!("An uknown parsing error happened"))
                }
            },
            _ => Err(format!("Command \"{}\" was parsed but could not be ran", command.as_str()))
        }
    }

    fn save(&mut self) {
        match &self.path {
            Some(path) => {
                let mut f = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(path).unwrap();
                let ser = serde_json::to_string(&self).unwrap();
                f.write_all(ser.as_bytes()).unwrap();
                f.flush().unwrap();
            },
            None => ()
        }
    }
}

#[derive(Serialize,Deserialize)]
struct DataBase {
    tables: HashMap<String, Table>,
}

impl DataBase {
    fn new() -> Self {
        Self {
            tables: HashMap::new()
        }
    }
    fn drop(&mut self, mut list: Pairs<Rule>) -> Result<Option<String>, String> {
        let table_name = list.next().unwrap().as_str();
        if self.tables.contains_key(table_name) {
            self.tables.remove(table_name);
            Ok(Some(format!("Table {} deleted.", table_name)))
        } else {
            Err(format!("!Failed to delete {} because it does not exist", table_name))
        }
    }
    fn alter(&mut self, mut list: Pairs<Rule>) -> Result<Option<String>, String> {
        let table_name = list.next().unwrap().as_str();
        match self.tables.get_mut(table_name) {
            Some(table) => {
                match table.alter(list) {
                    Ok(_) => Ok(Some(format!("Table {} modified", table_name))),
                    Err(_) => Err(format!("Table {} not modified", table_name))
                }
            }
            None => Err(format!("!Failed to modify table {} as it does not exist", table_name))
        }
    }
    fn select(&self, mut list: Pairs<Rule>) -> Result<Option<String>, String> {
        let select_args = list.next().unwrap();
        let table_name = list.nth(1).unwrap().as_str();
        match self.tables.get(table_name) {
            Some(table) => {
                table.select(select_args)
            },
            None => Err(format!("!Failed to query {} as it does not exist", table_name))
        }
    }
    fn create(&mut self, mut list: Pairs<Rule>) -> Result<Option<String>, String> {
        let name = list.next().unwrap().as_str();
        match self.tables.get(name) {
            Some(_) => Err(format!("!Failed to create table {} because it already exists.", name)),
            None => {
                self.tables.insert(String::from(name), Table::new(list));
                Ok(Some(format!("Table {} created.", name)))
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
enum SQLHeaderDef {
    Char(String, u32),
    Varchar(String, u32),
    Float(String),
    Int(String),
}

#[derive(Serialize, Deserialize)]
struct Table {
    header: Vec<SQLHeaderDef>,
}

impl Table {
    fn new(mut list: Pairs<Rule>) -> Self {
        let mut header: Vec<SQLHeaderDef> = Vec::new();
        for element in list.next().unwrap().into_inner() {
            match element.as_rule() {
                Rule::columnDef => {
                    let mut it = element.into_inner();
                    let name = it.next().unwrap().as_str();
                    let column = it.next().unwrap();
                    match column.as_rule() {
                        Rule::char => {
                            header.push(
                                SQLHeaderDef::Char(
                                    String::from(name), 
                                    column.into_inner().nth(1).unwrap().as_str().parse::<u32>().unwrap())
                            )
                        },
                        Rule::varchar => {
                            header.push(
                                SQLHeaderDef::Varchar(
                                    String::from(name), 
                                    column.into_inner().nth(1).unwrap().as_str().parse::<u32>().unwrap())
                            )
                        }
                        Rule::float => {
                            header.push(
                                SQLHeaderDef::Float(
                                    String::from(name)
                                )
                            )
                        }
                        Rule::int => {
                            header.push(
                                SQLHeaderDef::Int(
                                    String::from(name)
                                )
                            )
                        }
                        _ => (),
                    };
                }
                _ => ()
            }
        }
        Self {
            header
        }
    }
    
    fn select(&self, _list: Pair<Rule>) -> Result<Option<String>, String> {
        let mut out = String::new();
        for column in &self.header {
            match column {
                SQLHeaderDef::Int(name) => {
                    out.push_str(format!("{} int | ", name).as_str());
                },
                SQLHeaderDef::Char(name, size) => {
                    out.push_str(format!("{} char({}) | ", name, size).as_str());
                },
                SQLHeaderDef::Float(name) => {
                    out.push_str(format!("{} float | ", name).as_str());
                },
                SQLHeaderDef::Varchar(name, size) => {
                    out.push_str(format!("{} varchar({}) | ", name, size).as_str());
                },
            }
        }
        out.pop();
        out.pop();
        Ok(Some(out))
    }

    fn alter(&mut self, mut list: Pairs<Rule>) -> Result<Option<String>, String> {
        match list.next().unwrap().as_rule() {
            Rule::add => {
                for element in list.next().unwrap().into_inner() {
                    let mut it = element.into_inner();
                    let name = it.next().unwrap().as_str();
                    let column = it.next().unwrap();
                    match column.as_rule() {
                        Rule::char => {
                            self.header.push(
                                SQLHeaderDef::Char(
                                    String::from(name), 
                                    column.into_inner().nth(1).unwrap().as_str().parse::<u32>().unwrap())
                            )
                        },
                        Rule::varchar => {
                            self.header.push(
                                SQLHeaderDef::Varchar(
                                    String::from(name), 
                                    column.into_inner().nth(1).unwrap().as_str().parse::<u32>().unwrap())
                            )
                        }
                        Rule::float => {
                            self.header.push(
                                SQLHeaderDef::Float(
                                    String::from(name)
                                )
                            )
                        }
                        Rule::int => {
                            self.header.push(
                                SQLHeaderDef::Int(
                                    String::from(name)
                                )
                            )
                        }
                        _ => (),
                    };
                }
                Ok(Some(String::from("Table {} modified.")))
            },
            _ => Err(String::from("Table {} not modified"))
        }
    }
}
