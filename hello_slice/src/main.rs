fn main() {
    let s = "hello world";
    let first_word = get_first_word(s);
    println!("{}", first_word);

    let x = [1, 2, 3, 4, 5];
    let y = &x[..3];
    println!("{}", y[y.len() - 1]);
}

fn get_first_word(s: &str) -> &str {
    for (i, e) in s.chars().enumerate() {
        if e == ' ' {
            return &s[..i];
        }
    }

    &s[..]
}
