#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr_2 {
    V4(String),
    V6(String),
}

// #[derive(Debug)]
// enum Option<T> {
    
//     Some(T), // used to return a value
//     None     // used to return null, as Rust doesn't support
//              // the null keyword
// }

fn route(ip_kind: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    /*
        Illustration: Enum
    */

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /*
        Struct and Enum
    */
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // enum value
    let home = IpAddr_2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr_2::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback);
    println!("{:?}, {:?}", four, six);

    /*
        Enum option
    */ 
    let result = is_even(3);
    println!("{:?}", result);
    println!("{:?}", is_even(30));
}
fn is_even(no: i32) -> Option<bool> {
    if no%2 == 0 {
       Some(true)
    } else {
       None
    }
}