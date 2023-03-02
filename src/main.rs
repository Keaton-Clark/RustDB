use argh::FromArgs;
mod db;
use db::DBMS as DB;


#[derive(FromArgs)]
/// Rust SQL like database for CS 457
struct Args {

    /// open an interactive instance
    #[argh(switch, short = 'i')]
    interactive: bool,

    /// a SQL file to run
    #[argh(option, short = 'f')]
    file: Option<String>,

    /// a database to load or create
    #[argh(option, short = 'd')]
    database: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();
    let mut db = DB::new();
    
    match args.database {
        Some(_path) => {
            //load db into mem
        },
        None => ()
    }

    match args.file {
        Some(path) => {
            db.sql_from_file(path.as_str());
        },
        None => ()
    }

    match args.interactive {
        true => {
            db.interactive();
        },
        false => ()
    }
}
