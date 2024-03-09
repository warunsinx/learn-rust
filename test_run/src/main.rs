fn main() {
    fn add_x(x: f64) -> f64 {
        x + 1.0
    }

    let mut x: f64 = 1.5;

    let y: f64 = loop {
        println!("x is {}", x);
        x = add_x(x);
        if x > 5.0 {
            break x;
        }
    };

    const Z: [u32; 3] = [1, 2, 3];

    println!("y is {}", y);

    for n in Z {
        if n <= 2 {
            print!("{} ", n);
        } else {
            println!("{} ", n);
        }
    }

    while x < 10.0 {
        x = add_x(x);
    }

    println!("{}", x);
}
