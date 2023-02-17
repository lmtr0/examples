use std::{env, process::exit};
use backblaze_b2::{self as b2};

#[tokio::main]
async fn main() {
    if !dotenv::dotenv().is_ok() {
        println!("Failed to get environment .env")
    } 
    println!("Hello, world!");


    let (key, key_id, bucket_id) = (
        env::var("B2_CLIENT_KEY").ok(),
        env::var("B2_CLIENT_KEY_ID").ok(),
        env::var("BUCKET_ID").ok(),
    );

    if key.is_none() || key_id.is_none() || bucket_id.is_none() {
        println!("Authentication required");
        return;
    }

    // Client setup
    use hyper::Client;
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);


    let key = key.unwrap();
    let key_id = key_id.unwrap();
    let bucket_id = bucket_id.unwrap();

    let auth = b2::authorize_account(client, &key_id, &key).await;
        
    if auth.is_err() {
        println!("Failed to authorize account {}", auth.unwrap_err());
        return;
    };
    let mut auth = auth.unwrap();

    println!("* Obtaining authorization to upload a file.");
    let Ok(mut upload_auth) = b2::get_upload_authorization_by_id(
        &mut auth,
        &bucket_id
    ).await else {
        println!("Failed to obtain upload authorization");
        return;
    };

    println!("* Uploading file.");
    let Ok(upload_request) = b2::UploadFile::builder()
        .file_name("test").unwrap()
        .content_type("text/plain")
        // .sha1_checksum("81fe8bfe87576c3ecb22426f8e57847382917acf")
        .build() else {
            println!("Failed to build upload request");
            return;
        };

    let file = match b2::upload_file(&mut upload_auth, upload_request, b"abcd").await {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to upload file: {:?}", e);
            return;
        }
    };


    println!("* Downloading file.");
    let download_request = b2::DownloadFile::with_id(&file.file_id());
    let Ok( (downloaded, _headers)) = b2::download_file(&mut auth, download_request)
        .await else {
            println!("Failed to download file");
            return;
        };

    assert_eq!(downloaded, b"abcd");

    println!("\nUpload and download is complete.");
}
