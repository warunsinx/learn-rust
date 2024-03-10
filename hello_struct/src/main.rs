struct User {
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut warun = create_user(String::from("warunsinx@gmail.com"));

    warun.active = true;

    println!("{} {} {}", warun.email, warun.active, warun.sign_in_count);
}

fn create_user(email: String) -> User {
    User {
        email,
        active: false,
        sign_in_count: 0,
    }
}
