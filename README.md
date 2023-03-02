
# CS 457 part 1

## Rust Installation
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

## Rust Uninstallation

```rustup self uninstall```

## Running the project

```
$ cargo build 
$ cd target/debug
$ ./rust_db <args>
```
or
```
$ cargo run -- <args>
```
## Usage
```
Usage: rust_db [-i] [-f <file>] [-d <database>]

Rust SQL like database for CS 457

Options:
  -i, --interactive open an interactive instance
  -f, --file        a SQL file to run
  -d, --database    a database to load or create
  --help            display usage information
```

## Design
### Parsing and Lexing
The commands are parsed with a library called [pest](https://pest.rs/). The grammar can be found in sql.pest. The parser returns a iterable list of tokens.

### In-Memory Design
The DBMS struct stores a hashmap of all database structs and a string that is the key to the current database.

The database struct contains a hashmap of all the tables owned by that database.

The table struct contains a vector of the header columns. The columns are represented by an enum that contains the relevant data for that column. E.g. SQLHeaderDef::Varchar has a string that is the name of the column and an integer that is the size of the field.

### Storage Design


## Rust explanation
A fantastic free rust book can be found [here](https://doc.rust-lang.org/book/). Email me at keatonclark2@gmail.com if there is any confusion. In general rust is very similar to other Object-Oriented languages and you shouldn't have much trouble understanding the source code
### Lifetimes and Scope
The biggest difference between rust and most languages is ownership and lifetimes. Rust automatically deallocates variables when their owning scope ends. Ownership can be transfered when passing that variable to a function or they can be borrowed (no ownership tranfer) when passed with a `&`. More in-depth reading [here](https://doc.rust-lang.org/rust-by-example/scope.html). 

### Variables
Rust variables are immutable by default. Varible types are inferred but can be explictly defined with a colon and type.
```
let immutable_signed_8bit_integer: i8 = 17;

let mut mutable_unsigned_32bit_integer: u32 = 100;
mutable_unsigned_32bit_integer = 101;

let string: String = String::from("Hello, World!");
let ref_to_string: &str = string.to_str();
```

### Returning values
Rust has the `return` keyword but it is prefered that values are returned in the traditional functional format which means the last value of the function or block is returned. Remember that you must forgo the semicolon otherwise the function will return `()` which is similar to null

```
fn foo() -> i32 {
    let out = 17;
    out
}
```

### Object-Oriented
```
struct Foo {
    field_bar: i32,
}
impl Foo {
    fn get_field_bar(&self) -> i32 {
        self.field_bar
    }
}
fn main() {
    let foo = Foo {
        field_bar: 17
    };
    let bar = foo.get_field_bar();
}
```
### Option and Result
Option is a type in rust that allows you to have a optional field by either being Some(thing) or None. The type inside the Option is declared by `Option<type>`

Result is a type that allows you to have either a Ok(thing) or an Err(thing) and is the main way errors can be gracefully handled. Both the error and success can be different types by declaring `Result<ok_type, err_type>`

### Match
I use match a lot and it is very common in rust due to the amount of checking for Options and Results you must do. It is in essence a switch-case statement
```
let result: Result<i32, String> = something_that_returns_result();
match result {
    Ok(k) => {
        // if result is an ok, do something with the i32 which is now bound to 'k'
    },
    Err(e) => {
        // if result is an err, do something with the String which is now bound to 'e'
    }
}
```
Matches can also have a default case with `_ => {// expression}`
