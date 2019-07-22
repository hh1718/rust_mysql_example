mod dao;

fn main() {
    println!("{:?}", dao::select_user());
    println!("{:?}", dao::select_venue());
    println!("{:?}", dao::select_user_2());
    println!("{:?}", dao::select_venue_2());
}
