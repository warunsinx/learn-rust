fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    nums.push(69);

    match nums.get(20) {
        Some(value) => println!("{}", &value),
        None => println!("{}", &nums[nums.len() - 1]),
    };

    for y in &mut nums {
        *y += 10;
    }

    for x in &nums {
        println!("{}", &x);
    }

    enum Cell {
        Number(i32),
        Text(String),
    }

    let mut cells: Vec<Cell> = Vec::new();
    cells.push(Cell::Number(1));
    cells.push(Cell::Text(String::from("One")));

    let name = "Warun";
    for l in name.chars() {
        println!("{}", l);
    }

    use std::collections::HashMap;

    let mut liquidity: HashMap<String, i32> = HashMap::new();
    liquidity.insert("USDT/USDC".to_string(), 1000);
    liquidity.insert("ATA/USDT".to_string(), 2000);
    match liquidity.get("ATA/USDT") {
        Some(val) => println!("{}", val),
        None => return,
    }
    println!("{}", liquidity.get("USDT/USDC").copied().unwrap_or(0));

    for (k, v) in liquidity {
        println!("{} {}", k, v);
    }
}
