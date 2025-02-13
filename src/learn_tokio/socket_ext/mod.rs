use anyhow::{Context, Result};
use std::future::{Future, IntoFuture};
use std::os::unix::thread;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::time::error::Elapsed;
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
    // take away: if we want to use self to call a function in tokio::spawn, we should not pass self to a closure or future which has been pass into a self method.
    // Because it will cause a borrow checker error(double borrow because of it don't ensure that
    // self.op_with_timeout will end after fu, so it's not static).

    // Result: don't put a self Future in a self future call, it would lead to double borrow.
    async fn op_with_timeout_future<T, Fut>(&mut self, op_fut: Fut) -> Result<T>
    where
        Fut: Future<Output = std::io::Result<T>>,
    {
        let res = time::timeout(Duration::from_secs(1), op_fut)
            .await
            .map_err(anyhow::Error::from)??;
        Ok(res)
    }

    // Result: borrow a self in lambda function which return Future would not ensure that self outside would live longer than self inside the lambda function borrowing
    async fn op_with_timeout_func<T, Fn, Fut>(&mut self, op_fn: Fn) -> Result<T>
    where
        Fn: FnOnce(&mut Self) -> Fut,
        Fut: Future<Output = std::io::Result<T>>,
    {
        let res = time::timeout(Duration::from_secs(1), op_fn(self))
            .await
            .map_err(anyhow::Error::from)??;
        Ok(res)
    }

    async fn op_with_timeout<'a, T, F, Fut>(&'a mut self, op: F) -> Result<T, std::io::Error>
    where
        F: FnOnce(&mut Self) -> Fut,
        Fut: Future<Output = io::Result<T>> + 'a,
    {
        timeout(Duration::from_secs(1), op(self))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "test"))?
    }

    // async fn read_u32_(&mut self, duration: time::Duration) {
    //     self.op_with_timeout::<u32, _, _>(|s| s.read_u32()).await;
    // }

    // async fn read_u32_future_double_borrow(&mut self, duration: time::Duration) {
    //     self.op_with_timeout_future(self.read_u32());
    // }
    // async fn read_u32_double_borrow(&mut self, duration: time::Duration) {
    //     let fu = self.read_u32();
    //     self.op_with_timeout_func(|s| s.read_u32());
    // }

    async fn read_u32_timeout(&mut self, duration: time::Duration) -> Result<u32> {
        // let fu = self.read_u32();
        // self.op_with_timeout(fu)
        time::timeout(duration, self.read_u32())
            .await?
            .context("Timeout while read u32")
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
