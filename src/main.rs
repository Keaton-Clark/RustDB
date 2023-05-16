// Keaton Clark
// 03/05/23
mod db;
use db::DBMS as DB;



fn main() {
    
    let mut db = DB::new(Some(String::from("dbms")));
    db.interactive()
}
