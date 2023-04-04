use std::{
    io,
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
};

use chrono::{Duration, NaiveDate};

mod keygen;
mod web;

fn main() {
    let web_thread = thread::spawn(|| {
        web::create_server(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 80)).unwrap();
    });
    print!("Input your name (default name: user):");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    if name.trim().is_empty() {
        name = "user".into();
    }

    let valid_until = NaiveDate::from_ymd_opt(2020, 12, 12).unwrap() + Duration::days(3650);
    let mut key = keygen::Password::new();
    key.generate_key(name.trim(), valid_until, 1).unwrap();

    println!("Your name:\n\t{}", name.trim());
    println!("Your password:\n\t{}", key);
    println!("Expiration:\n\t{}", valid_until.format("%Y-%m-%d"));

    web_thread.join().unwrap();
}
