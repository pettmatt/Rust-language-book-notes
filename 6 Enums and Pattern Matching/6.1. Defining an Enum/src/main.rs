enum IpAddrKind { V4, V6 }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {
    // Some code here
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // I know that this code doesn't compile
    // I'm just too lazy to fix it at this moment
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("Hello, world!");
}