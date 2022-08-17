//! The buggers chat protocal.
//! A protocal to transfer data between buggerschat server and buggerschat client.
//! 
//! Data format:
//! ```plain
//! 0x7f 0x2a [TYPE]  - where TYPE has two values: 0x00 for String and 0x01 for logout, 0x02 for idle.
//! For string, each byte are in the following format:
//! 0x27 [byte]
//! And end by:
//! 0x00 0x00
//! ```

use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

pub struct BuggersChatProtocal;

pub enum BuggersChatProtocalMessageType {
    String(String),
    Disconnect,
    Idle,
}

impl BuggersChatProtocal {
    pub fn read_message(stream: &mut TcpStream) -> std::io::Result<BuggersChatProtocalMessageType> {
        let mut header = [0_u8; 3];
        if let Ok(_) = stream.read(&mut header) {
            // println!("Read header {:x?}", header);
            if header[0] == 0x7f && header[1] == 0x2a {
                match header[2] {
                    0x00 => {
                        let mut buffer = [0_u8; 2];
                        let mut result = Vec::<u8>::new();
                        loop {
                            stream.read(&mut buffer)?;
                            match buffer[0] {
                                0x27 => {
                                    result.push(buffer[1]);
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        Ok(BuggersChatProtocalMessageType::String(String::from_utf8_lossy(&result).to_string()))
                    }
                    0x01 => {
                        Ok(BuggersChatProtocalMessageType::Disconnect)
                    }
                    _ => {
                        Ok(BuggersChatProtocalMessageType::Idle)
                    }
                }
            } else {
                Ok(BuggersChatProtocalMessageType::Idle)
            }
        } else {
            Ok(BuggersChatProtocalMessageType::Idle)
        }
    }
    pub fn write_string(stream: &mut TcpStream, s: &str) -> std::io::Result<()> {
        let mut v = Vec::<u8>::new();
        v.push(0x7f);
        v.push(0x2a);
        v.push(0x00);

        for i in s.bytes() {
            v.push(0x27);
            v.push(i);
        }
        v.push(0x00);
        v.push(0x00);

        stream.write(&v[..])?;

        Ok(())
    }
    pub fn disconnect(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write(&[0x7f, 0x2a, 0x01])?;
        Ok(())
    }
    pub fn make_idle(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write(&[0x7f, 0x2a, 0x02])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BuggersChatProtocal;
    use super::BuggersChatProtocalMessageType;

    #[test]
    fn it_works() {
    }
}
