use mysql;
use std::env;

fn pool() -> mysql::Pool {
  let user: String = env::var("RUST_DB_USER").unwrap();
  let pw: String = env::var("RUST_DB_PASS").unwrap();
  let host: String = env::var("RUST_DB_HOST").unwrap();
  let url = "mysql://".to_string() + &user + &":".to_string() + &pw.to_string() + &"@".to_string() + &host + &"/mysql".to_string();
  mysql::Pool::new(url).unwrap()
}
pub fn select<T, F>(query: String, row_to_struct: F) -> Vec<T> where F: Fn(mysql::Row) -> T{
  let pool = pool();
  pool.prep_exec(query, ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| row_to_struct(row)).collect()
    }).unwrap()
}

