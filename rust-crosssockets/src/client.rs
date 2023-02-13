mod pb {
    tonic::include_proto!("hello.v1");
}

use pb::{HelloMessage, hello_client::HelloClient};
use tonic::{transport::{Endpoint, Uri}};
use tower::service_fn;

const PIPE_NAME: &str = r"\\.\pipe\named-pipe-single-client";

#[cfg(windows)]
async fn connect_to_pipe(_: Uri) -> std::io::Result<tokio::net::windows::named_pipe::NamedPipeClient> {
    use tokio::net::windows::named_pipe::{ClientOptions};
    
    let client = ClientOptions::new().open(PIPE_NAME)?;
    match client.readable().await {
        Err(_e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read pipe")),
        Ok(()) => Ok(client),
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    #[cfg(target_os = "windows")]
    {
        let client = Endpoint::try_from("http://d").unwrap()
            .connect_with_connector(service_fn(connect_to_pipe))
            .await.expect("Failed to create client");

        let mut client = HelloClient::new(client);
        let reply = client.hello(HelloMessage {
            message: format!("Hello from the client")
        }).await.expect("Failed to talk to server");
    
        dbg!(reply);
    }


    Ok(())
}
