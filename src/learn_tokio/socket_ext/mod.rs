use anyhow::{Context, Result};
use std::os::unix::thread;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time::{self, timeout, Duration};

// 啟動一個 dummy server，在接收到連線後延遲回應
async fn start_dummy_server(addr: &str) -> Result<TcpStream> {
    let listener = TcpListener::bind(addr)
        .await
        .context("Failed to bind listener")?;
    dbg!("Dummy server listening on {}", addr);
    let (tcp_stream, addr) = listener
        .accept()
        .await
        .context("Failed to accept connection")?;
    Ok(tcp_stream)
}

pub(crate) trait SocketExt: AsyncReadExt + AsyncWriteExt + Unpin {
    async fn op_with_timeout<T, F>(&mut self, op_fut: F) {}

    async fn read_u32_timeout(&mut self, duration: time::Duration) -> Result<u32> {
        time::timeout(duration, self.read_u32())
            .await?
            .context("Timeout while reading u32")
    }
    async fn read_u8_timeout(&mut self, duration: Duration) -> Result<u8> {
        dbg!("reading u8");
        timeout(duration, self.read_u8())
            .await?
            .context("Timeout while reading u8")
    }
    async fn read_exact_timeout(&mut self, duration: Duration, buf: &mut Vec<u8>) -> Result<usize> {
        timeout(duration, self.read_exact(buf))
            .await?
            .context("Timeout while reading exact")
    }
}

impl<T: AsyncReadExt + AsyncWriteExt + Unpin> SocketExt for T {}

pub(crate) async fn test_read_u8_timeout_error_timeout() -> Result<u8> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        // try to connect to the server
        let mut tcp_stream_client = TcpStream::connect("127.0.0.1:12345").await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        tcp_stream_client.write_u8(1).await.unwrap();
    });
    dbg!("starting dummy server");
    let mut tcp_stream_server = start_dummy_server("127.0.0.1:12345")
        .await
        .context("Failed to start dummy server")
        .unwrap();
    dbg!("dummy server started");
    dbg!("waiting for timeout");
    let len = tcp_stream_server
        .read_u8_timeout(time::Duration::from_secs(1))
        .await
        .context("Failed to read u8 with timeout")?;
    Ok(len)
}
