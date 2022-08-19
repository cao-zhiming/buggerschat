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

use std::io::Error;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

/// Contains methods to operate Buggers Chat Protocals
pub struct BuggersChatProtocal;

/// The message's type.
#[derive(Debug, Clone)]
pub enum BuggersChatProtocalMessageType {
    String(String),
    Disconnect,
    Idle,
}

impl BuggersChatProtocal {

    /// Read the message from the stream.
    pub fn read_message(stream: &mut TcpStream) -> std::io::Result<BuggersChatProtocalMessageType> {

        // Read the header first
        let mut header = [0_u8; 3];

        // If we read the header succesfully,
        stream.read(&mut header)?;

        // Check the headers.
        if header[0] == 0x7f && header[1] == 0x2a {
            match header[2] {
                0x00 => {
                    // Read each charater and get the string.
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
                    // Disconnect signal.
                    Ok(BuggersChatProtocalMessageType::Disconnect)
                }
                _ => {
                    // Otherwise, make idle.
                    Ok(BuggersChatProtocalMessageType::Idle)
                }
            }
        } else {
            Err(Error::new(std::io::ErrorKind::BrokenPipe, "Failed to recv"))
        }
    }


    /// Read asynchronoly.
    pub fn read_message_async<F: FnOnce(std::io::Result<BuggersChatProtocalMessageType>) -> () + Send + 'static>(stream: Arc<Mutex<TcpStream>>, on_complete: F) {
        std::thread::spawn(move || {
            loop {
                if let Ok(mut stream) = stream.try_lock() {
                    let msg = Self::read_message(&mut stream);
                    on_complete(msg);
                    break;
                }
            }
        });
    }

    /// Write the string to the `stream`.
    pub fn write_string(stream: &mut TcpStream, s: &str) -> std::io::Result<()> {

        // Write the header first.
        let mut v = Vec::<u8>::new();
        v.push(0x7f);
        v.push(0x2a);
        v.push(0x00);

        // Then write each characters.
        for i in s.bytes() {
            v.push(0x27);
            v.push(i);
        }
        v.push(0x00);
        v.push(0x00);

        stream.write(&v[..])?;

        Ok(())
    }

    /// Send a disconnect signal to the given stream.
    pub fn disconnect(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write(&[0x7f, 0x2a, 0x01])?;
        Ok(())
    }

    /// Make the given stream idle.
    pub fn make_idle(stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write(&[0x7f, 0x2a, 0x02])?;
        Ok(())
    }
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
