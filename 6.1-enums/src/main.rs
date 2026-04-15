struct IpV4Addr {
    // Something!
}

struct IpV6Addr {
    // Something!
}

enum IpAddrKind {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let home = IpAddrKind::V4(IpV4Addr {});
    let loopback = IpAddrKind::V6(IpV6Addr {});

    // can't get the inner values yet :)
}
