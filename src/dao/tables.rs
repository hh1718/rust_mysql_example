#[derive(Debug, PartialEq, Eq)]
pub struct UserRow {
    pub id: i32,
    pub name: Option<String>
}

#[derive(Debug)]
pub struct User {
}

#[derive(Debug, PartialEq, Eq)]
pub struct VenueRow {
    pub id: i32,
    pub name: Option<String>,
    pub address: Option<String>
}

#[derive(Debug)]
pub struct Venue {
}