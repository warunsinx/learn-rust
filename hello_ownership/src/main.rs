fn main() {
    let x = String::from("x");
    let y = take_owner_and_return(x);
    use_ref(&y);
    println!("{}", y);

    let mut a = String::from("hello");
    use_mut_ref(&mut a);
    println!("{}", a);
}

fn take_owner_and_return(s: String) -> String {
    s
}

fn use_ref(s: &String) {
    println!("{}", s);
}

fn use_mut_ref(s: &mut String) {
    s.push_str(", world !");
}
