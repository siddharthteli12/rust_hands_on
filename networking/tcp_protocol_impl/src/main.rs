use std::io::Error;

use tun_tap::Mode;


fn main() -> Result<(), Error> {
    let nic = tun_tap::Iface::new("tun0", Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    let nbytes = nic.recv(&mut buffer)?;
    eprint!("read {} bytes: {:x}", nbytes, &buffer[..nbytes]);
    Ok(())
}   
