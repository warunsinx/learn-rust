struct User {
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct ColorX(u32, u32, u32);

#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn size(&self) -> u32 {
        self.height * self.width
    }

    fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Square {
    fn is_gt(&self, sq_ref: &Square) -> bool {
        if self.size() > sq_ref.size() {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut warun = create_user(String::from("warunsinx@gmail.com"));
    warun.active = true;

    println!("{} {} {}", warun.email, warun.active, warun.sign_in_count);

    let x = create_user(String::from("x@x.com"));
    let mut y = User { ..x };

    y.active = true;
    y.sign_in_count = y.sign_in_count + 1;
    y.email = String::from("hello@hello.com");

    println!("{} {} {}", y.email, y.active, y.sign_in_count);

    let red = ColorX(5, 5, 5);

    println!("{} {} {}", red.0, red.1, red.2,);

    let sq1 = Square {
        width: dbg!(50 * 2),
        height: 100,
    };

    dbg!(&sq1);

    println!("{}", sq1.size());

    let sq2 = Square::new(500);

    println!("{}", sq2.size());

    println!("{}", sq1.is_gt(&sq2));
}

fn create_user(email: String) -> User {
    User {
        email,
        active: false,
        sign_in_count: 0,
    }
}
