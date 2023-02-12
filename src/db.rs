use std::collections::BTreeMap;
use std::fs;
use std::io::{Write, stdout, stdin};
use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
use std::marker::PhantomData;

#[derive(Parser)]
#[grammar = "sql.pest"]
struct SQLParser;

pub struct DataBase {
    error: Option<String>,
}

impl DataBase {
    const PROMPT: &str = " > ";
    pub fn new() -> Self {
        DataBase {
            error: None,
        }
    }
    pub fn interactive(&mut self) {
        let mut line = String::new();
        loop {
            stdout().write(DataBase::PROMPT.as_bytes()).unwrap();
            stdout().flush().unwrap();
            stdin().read_line(&mut line).unwrap();
            self.parse(line.as_str());
            line.clear();
        }
    }
    pub fn sql_from_file(&mut self, path: &str) {
        match fs::read_to_string(path) {
            Err(e) => println!("{}", e),
            Ok(k) => {
                self.parse(k.as_str());
            }
        }
    }
    fn parse(&mut self, string: &str) {
        match SQLParser::parse(Rule::SQL, string) {
        Ok(k) => {
                for line in k {
                    println!("{:?}", line.as_span());
                }
            },
        Err(e) => {
                println!("{e}");
            }
        }
    }
}
/*
#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"\--(.*)", logos::skip, priority = 10)]

    #[token(";", priority = 5)]
    Semicolon,

    #[token(",", priority = 5)]
    Comma,

    #[token("*", priority = 5)]
    Star,

    #[token("CREATE", priority = 5)]
    Create,

    #[token("USE", priority = 5)]
    Use,

    #[token("DROP", priority = 5)]
    Drop,

    #[token("TABLE", priority = 5)]
    Table,

    #[token("SELECT", priority = 5)]
    Select,

    #[token("FROM", priority = 5)]
    From,

    #[token(".EXIT", priority = 5)]
    Exit,

    #[token("int", priority = 5)]
    Int,

    #[token("float", priority = 5)]
    Float,

    #[token("char", priority = 5)]
    Char,

    #[token("varchar", priority = 5)]
    Varchar,


    #[token("(", priority = 5)]
    OpenParenthesis,

    #[token(")", priority = 5)]
    CloseParenthesis,

    #[token("DATABASE", priority = 5)]
    Database,

    #[token("ALTER", priority = 5)]
    Alter,

    #[regex(r"[a-zA-Z0-9_]+", priority = 3)]
    Symbol,

    #[regex(r"[0-9]+", priority = 4)]
    Number,

    #[error]
    #[regex(r"[ \n]+", logos::skip, priority = 10)]
    Error,
}*/
