use mysql;
mod db;
mod tables;

pub fn select_user() -> Vec<tables::UserRow> {
    let query: String = "SELECT id, name from lms.user ".to_string();
    db::select(query, |row| {
        let (id, name) = mysql::from_row::<(i32, Option<String>)>(row);
        tables::UserRow {
            id: id,
            name: name
        }
    })
}
pub fn select_venue() -> Vec<tables::VenueRow> {
    let query: String = "SELECT id, name, address from lms.venue ".to_string();
    db::select(query, |row| {
        let (id, name, address) = mysql::from_row::<(i32, Option<String>, Option<String>)>(row);
        tables::VenueRow {
            id: id,
            name: name,
            address: address
        }
    })
}




pub trait Dao<T> {
    fn select() -> Vec<T>;
}

impl Dao<tables::UserRow> for tables::User {
    fn select() -> Vec<tables::UserRow> {
        let query: String = "SELECT id, name from lms.user ".to_string();
        db::select(query, |row| {
            let (id, name) = mysql::from_row::<(i32, Option<String>)>(row);
            tables::UserRow {
                id: id,
                name: name
            }
        })
    }
}
impl Dao<tables::VenueRow> for tables::Venue {
    fn select() -> Vec<tables::VenueRow> {
        let query: String = "SELECT id, name, address from lms.venue ".to_string();
        db::select(query, |row| {
            let (id, name, address) = mysql::from_row::<(i32, Option<String>, Option<String>)>(row);
            tables::VenueRow {
                id: id,
                name: name,
                address: address
            }
        })
    }
}

pub fn select_user_2() -> Vec<tables::UserRow> {
    tables::User::select()
}
pub fn select_venue_2() -> Vec<tables::VenueRow> {
    tables::Venue::select()
}



