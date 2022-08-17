use std::{net::TcpStream, io::Write};

/// Write a string to a buggerschat server
pub fn write_string_to(stream: &mut TcpStream, data: &str) -> std::io::Result<()> {
    for i in data.as_bytes() {
        stream.write(&[0x27, *i])?;
    }
    stream.write(&[0x00, 0x00])?;
    Ok(())
}
