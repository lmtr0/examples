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
        use tokio::net::windows::named_pipe::{ClientOptions};

        const PIPE_NAME: &str = r"\\.\pipe\named-pipe-single-client";
        let client = tokio::spawn(async move {
            // There's no need to use a connect loop here, since we know that the
            // server is already up - `open` was called before spawning any of the
            // tasks.
            let client = ClientOptions::new().open(PIPE_NAME)?;

            let mut client = BufReader::new(client);

            let mut buf = String::new();
            client.write_all(b"ping\n").await?;
            client.read_line(&mut buf).await?;
            Ok::<_, io::Error>(buf)
        });

        let (client,) = tokio::try_join!(client)?;
        println!("Client: \"{}\"", client?);
    }

    Ok(())
}
