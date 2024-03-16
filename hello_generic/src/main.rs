struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn p_x(&self) {
        println!("x");
    }

    fn mix_up<T2, U2>(self, point: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

impl Point<i32, i32> {
    fn p_y(&self) {
        println!("y");
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let letters = vec!['a', 'b', 'c', 'x'];
    println!("{}", find_largest(&nums));
    println!("{}", find_largest(&letters));

    let start = Point { x: 5, y: 5 };
    let end = Point { x: 10, y: 4.5 };

    println!(
        "Start at {}, {} | End at {}, {}",
        &start.x, &start.y, &end.x, &end.y
    );

    end.p_x();
    start.p_y();

    let mix = start.mix_up(end);

    println!("{} {}", mix.x, mix.y);
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_num = &list[0];

    for num in list {
        if num > largest_num {
            largest_num = num
        }
    }

    largest_num
}
