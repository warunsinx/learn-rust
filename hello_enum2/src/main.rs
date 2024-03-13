fn main() {
    let x: Option<i32> = Some(5);

    fn format_x(x: Option<i32>) -> i32 {
        match x {
            Some(x) => x,
            _ => 0,
        }
    }

    println!("{}", format_x(x));

    let y = Some(10);

    if let Some(val) = y {
        println!("{}", val);
    }
}
