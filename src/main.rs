fn main() {
    enum_practice()
}

fn enum_practice() {
    println!("Hello, world!");

    let four: IPAddrKind = IPAddrKind::V4;
    let six: IPAddrKind = IPAddrKind::V6;

    route(four);
    route(six);

    let four_v2: IPAddrKindV2 = IPAddrKindV2::V4(8, 8, 8, 8);
    let six_v2: IPAddrKindV2 = IPAddrKindV2::V6(String::from("::1"));
}

enum IPAddrKind {
    V4, V6
}

struct IPAddr {
    kind: IPAddrKind,
    address: String
}

fn route(ip_kind: IPAddrKind) {
    // Doing something here

    if matches!(ip_kind, IPAddrKind::V4) {
        let home : IPAddr = IPAddr { kind: ip_kind, address: String::from("185.233.86.81") };
        println!("{}", home.address);
    } else {
        let loopback : IPAddr = IPAddr { kind: ip_kind, address: String::from("::1") };
        println!("{}", loopback.address);
    }
}

enum IPAddrKindV2 {
    V4(i32, i32, i32, i32),
    V6(String)
}