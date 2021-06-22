use core::pin::Pin;
use core::task::{Context, Poll};
use futures::SinkExt;
use std::io::Write;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf, Result};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub const LONG_LINE: &str =
    "------------------------------------------------------------------------\r";

pub struct IO<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    io: Framed<T, LinesCodec>,
}

impl<T> IO<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: T) -> Self {
        let codec = LinesCodec::new_with_max_length(128);
        let framed = Framed::new(stream, codec);
        IO { io: framed }
    }

    pub async fn write(&mut self, text: &str) {
        let _ = self.io.send(format!("{}\r", text).as_str()).await;
    }

    pub async fn read(&mut self) -> String {
        while let Some(result) = self.io.next().await {
            if let Ok(data) = result {
                return data;
            }
        }
        "".to_owned()
    }
}

pub struct StdInOut {}

impl StdInOut {
    pub fn new() -> StdInOut {
        StdInOut {}
    }
}
impl AsyncRead for StdInOut {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        buf.put_slice(&line.as_bytes());
        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for StdInOut {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        let _ = std::io::stdout().write_all(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        let _ = std::io::stdout().flush();
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }
}
