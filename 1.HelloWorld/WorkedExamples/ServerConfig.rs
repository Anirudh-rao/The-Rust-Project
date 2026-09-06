#[derive(Debug)]
struct IpAddress {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Debug)]
struct ServerConfig {
    name: String,
    ip: IpAddress,
    port: u16,
}

fn main() {
    let config = ServerConfig {
        name: String::from("Production_DB"),
        ip: IpAddress { a: 192, b: 168, c: 1, d: 50 },
        port: 5432,
    };

    // Compact print
    println!("Compact: {:?}", config);

    // Pretty-printed format
    println!("Pretty:
{:#?}", config);
}