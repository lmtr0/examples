use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // loop {
    //     let client = socket.accept();
    // }

    #[cfg(target_os = "windows")]
    {
        use std::io;
        use tokio::io::AsyncWriteExt;
        use tokio::io::{AsyncBufReadExt, BufReader};
        use tokio::net::windows::named_pipe::{ServerOptions};

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
        loop {
            let server = opt.create(PIPE_NAME)?;
            server.connect().await?;

            let mut server_reader = BufReader::new(server);

            let mut buf = String::new();
            server_reader.read_line(&mut buf).await?;
            server_reader.write_all(b"pong\n").await?;
            println!("Waiting 3 seconds");
            std::thread::sleep(Duration::from_secs(3));

            println!("server \"{}\"", buf);
            
        }

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
