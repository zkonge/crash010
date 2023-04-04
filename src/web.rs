use std::{
    io::{Error, Read, Write},
    net::{SocketAddr, TcpListener},
};

const RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type:text/html\r\n\r\n<ss>valid</ss>";

pub fn create_server(bind: SocketAddr) -> Result<(), Error> {
    let listener = TcpListener::bind(bind)?;

    let (mut stream, _) = listener.accept()?;

    let mut buf = [0u8; 1024];
    let _ = stream.read(&mut buf).expect("Receive http request failed.");
    stream
        .write_all(RESPONSE.as_bytes())
        .expect("Send http respond failed.");
    stream.flush().expect("Send http respond failed.");

    Ok(())
}
