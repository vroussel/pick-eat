use std::net::SocketAddr;

use pickeat_server::run;
use tokio::net::TcpListener;

#[cfg(test)]
pub(crate) async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Unable to bind on address");
    let addr = listener
        .local_addr()
        .expect("Unable to retrieve server port");
    tokio::spawn(run(listener));
    addr
}
