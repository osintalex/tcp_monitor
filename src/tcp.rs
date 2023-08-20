use std::net::TcpStream;

pub fn connect(address: &str) -> bool {
    let stream = TcpStream::connect(address);
    match stream {
        Ok(_) => {
            log::info!("Address {} is accessible over TCP", address);
            return true;
        }
        Err(err) => {
            log::warn!("Address {} is unreachable. Error: {}", address, err);
            return false;
        }
    }
}
