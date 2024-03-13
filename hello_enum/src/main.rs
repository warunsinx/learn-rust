use std::fmt::Debug;

#[derive(Debug)]
enum IpType {
    Ipv4(String),
    Ipv6(String),
}

impl IpType {
    fn get_ip(&self) {
        print!("{:?} ", &self);
    }
}

fn is_ipv4(ip: &IpType) -> bool {
    match ip {
        IpType::Ipv4(state) => {
            if state == "127.0.0.1" {
                true
            } else {
                false
            }
        }
        IpType::Ipv6(_) => false,
    }
}

fn main() {
    let home = IpType::Ipv4(String::from("127.0.0.1"));
    let work = IpType::Ipv6(String::from("0x123"));

    println!("{:?} {}", home, is_ipv4(&home));
    work.get_ip();
    println!("{}", is_ipv4(&work));
}
