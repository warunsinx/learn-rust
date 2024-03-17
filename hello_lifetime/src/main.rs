struct User<'a> {
    email: &'a str,
}

impl<'a> User<'a> {
    fn greet(&self, message: &str) -> String {
        format!("{}: {}", self.email, message)
    }

    fn test_life(a: &'a str, b: &'a str) -> String {
        return format!("{}{}", a, b);
    }
}

fn main() {
    let a = String::from("x");
    let result;
    {
        let b = String::from("b");
        result = logest(&a, &b);
    }
    println!("{}", result);

    let mail_one;

    let mail = "warunsinx@gmail.com".to_string();
    mail_one = &mail;

    let warun = User { email: &mail_one };
    println!("{}", warun.email);
}

fn logest<'a>(_a: &'a str, _b: &str) -> &'a str {
    "z"
}
