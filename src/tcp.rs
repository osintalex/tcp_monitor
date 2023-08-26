use std::net::TcpStream;

/// Attempt a tcp connection to assess connectivity.
///
/// # Arguments
///
/// * `address` - string slice of the IPv4 or IPv6 address
///
pub fn connect(address: &str) -> bool {
    let stream = TcpStream::connect(address);
    match stream {
        Ok(_) => {
            log::info!("Address {} is accessible over TCP", address);
            true
        }
        Err(err) => {
            log::warn!("Address {} is unreachable. Error: {}", address, err);
            false
        }
    }
}
