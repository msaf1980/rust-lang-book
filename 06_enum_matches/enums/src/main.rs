enum IpVersion {
    V4,
    V6,
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn is_ipv4(ip: &String) -> bool {
    let split = ip.split('.');
    let mut i: u8 = 0;
    for s in split {
        match s.parse::<u8>() {
            Ok(n) => {
                match i {
                    0 | 3 => {
                        if n == 0 || n > 254 {
                            return false;
                        }
                    }
                    4 => {
                        return false;
                    }
                    _ => {
                        if n > 254 {
                            return false;
                        }
                    }
                }
            }
            Err(e) => {
                return false;
            }
        }
        i += 1;
    }
    if i != 4 {
        return false;
    } else {
        return true;
    }
}

struct IpAddr {
    vers: IpVersion,
    address: String,
}

impl IpAddr {
    fn ip_v4(ip: &String) -> Result<IpAddr, &'static str> {
        if ! is_ipv4(ip) {
            return Err("format error");
        }
        Ok(IpAddr {
            vers: IpVersion::V4,
            address: ip.clone(),
        })
    }

    fn ip_v6(ip: &String) -> Result<IpAddr, &'static str> {
        Ok(IpAddr {
            vers: IpVersion::V6,
            address: ip.clone(),
        })
    }
}

fn main() {
    let str = "127.0.0.1".to_owned();
    match IpAddr::ip_v4( &str ) {
        Ok(ip) => {
            println!("{}", ip.address);
        }
        Err(e) => {
            println!("{}: {}", str, e);
        }
    }

    let str = "127.0.0.256".to_owned();
    match IpAddr::ip_v4( &str ) {
        Ok(ip) => {
            println!("{}", ip.address);
        }
        Err(e) => {
            println!("{}: {}", str, e);
        }
    }

    let str = "::1".to_owned();
    let loopback = IpAddr::ip_v6( &str ).unwrap();

    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &rows {
        match i {
            SpreadsheetCell::Int(n) => println!("int {}", n),
            SpreadsheetCell::Float(n) => println!("float {}", n),
            SpreadsheetCell::Text(n) => println!("str {}", n),
        }
    }
}
