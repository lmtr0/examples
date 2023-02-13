mod pb {
    tonic::include_proto!("hello.v1");
}

use pb::{
    hello_server::{Hello, HelloServer},
    HelloMessage,
};
use tonic::{Request, Response, Status};


#[derive(Debug, Default)]
struct Server {}

#[tonic::async_trait]
impl Hello for Server {
    async fn hello(&self, req: Request<HelloMessage>) -> Result<Response<HelloMessage>, Status> {
        dbg!(req);
        Ok(Response::new(HelloMessage {
            message: format!("Received message"),
        }))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // loop {
    //     let client = socket.accept();
    // }

    #[cfg(windows)]
    {
        // use tokio::io::AsyncWriteExt;
        // use tokio::io::{AsyncBufReadExt, BufReader};
        use tokio::net::windows::named_pipe::ServerOptions;
        use crate::windows::WindowsStream;


        const PIPE_NAME: &str = r"\\.\pipe\named-pipe-single-client";

        // let server = ServerOptions::new().create(PIPE_NAME)?;
        // let incoming = {
        //     async_stream::stream! {
        //         loop {
        //             server.connect().await;
        //             yield BufReader::new(server);
        //         }
        //     }
        // };

        // Note: we wait for a client to connect.
        let opt = ServerOptions::new();
        let incoming = {
            async_stream::stream! {
                loop {
                    // Pipes are "single threaded" only one client and server at a this point, but no limit on the creation of server listeners Hm!
                    let server = opt.create(PIPE_NAME)?;

                    match server.connect().await {
                        Ok(()) => {},
                        Err(_) => yield Err(std::io::Error::new(std::io::ErrorKind::Other, "Client failed to connect to the server")),
                    };
                    
                    yield Ok(WindowsStream::new(server));
                }
            }
        };

        // let addr = "[::1]:50051".parse().unwrap();
        let greeter = Server::default();

        println!("GreeterServer listening on {}", PIPE_NAME);

        tonic::transport::Server::builder()
            .add_service(HelloServer::new(greeter))
            .serve_with_incoming(incoming)
            // .serve(addr)
            .await
            .expect("Failed to start up server");

        // let client = tokio::spawn(async move {
        //     // There's no need to use a connect loop here, since we know that the
        //     // server is already up - `open` was called before spawning any of the
        //     // tasks.
        //     let client = ClientOptions::new().open(PIPE_NAME)?;

        //     let mut client = BufReader::new(client);

        //     let mut buf = String::new();
        //     client.write_all(b"ping\n").await?;
        //     client.read_line(&mut buf).await?;
        //     Ok::<_, io::Error>(buf)
        // });
    }

    Ok(())
}

#[cfg(windows)]
mod windows {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;
    use tokio::net::windows::named_pipe::NamedPipeServer;

    #[derive(Debug)]
    pub struct WindowsStream(pub NamedPipeServer);

    impl WindowsStream { 
        pub fn new(server: NamedPipeServer) -> WindowsStream {
            WindowsStream(server)
        }
    }

    impl Connected for WindowsStream {
        type ConnectInfo = ();

        fn connect_info(&self) -> Self::ConnectInfo {
            ()
        }
    }

    impl AsyncRead for WindowsStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for WindowsStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}
