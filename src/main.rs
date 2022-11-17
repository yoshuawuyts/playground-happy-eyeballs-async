use async_std::io::prelude::*;
use futures_concurrency::prelude::*;
use futures_time::prelude::*;

use async_std::net::TcpStream;
use futures_time::time::Duration;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut socket = open_tcp_socket("https://debian.org", 80, 3).await?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await?;
    println!("{buf}");
    Ok(())
}

/// Happy eyeballs algorithm!
///
/// 1. Create an iterator for the number of attempts
/// 2. Create a `TcpStream` for each attempt
/// 3. Delay the start of each tcp stream by `N` seconds
/// 4. Collect into a vector of futures
/// 5. Concurrently execute all futures using "race_ok" semantics -
///    on failure, try the next future. Return on success or if all futures
///    fail.
/// 6. Set a timeout for our entire futures construct, so we stop all at the same time.
async fn open_tcp_socket(
    addr: &str,
    port: u16,
    attempts: u64,
) -> Result<TcpStream, Box<dyn Error + Send + Sync + 'static>> {
    let socket = (0..attempts)
        .map(|i| TcpStream::connect((addr, port)).delay(Duration::from_secs(i)))
        .collect::<Vec<_>>()
        .race_ok()
        .timeout(Duration::from_secs(attempts + 1))
        .await??;
    Ok(socket)
}
