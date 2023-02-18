#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(String),
}

impl IpAddressKind {
    fn show_address(&self) {
        println!("Ip address is {:?}", self);
    }
}

enum Coin {
    Ruppe,
    Five,
    Ten,
}

fn main() {
    let ip_add_four = IpAddressKind::V4(String::from("127.0.0.1"));
    let ip_add_six = IpAddressKind::V6(String::from("adladladhlajdi"));

    ip_add_four.show_address();
    ip_add_six.show_address();

    let coin = Coin::Ruppe;

    println!("Value of coin is {}", get_coin_value(coin));
}

fn get_coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Five => 5,
        Coin::Ruppe => 1,
        Coin::Ten => 10,
    }
}
