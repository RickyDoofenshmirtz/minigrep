use aws_sdk_s3::{ByteStream, Client, Config, Region};
use std::env;
use std::fs::File;
use std::io::Read;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load AWS credentials and region
    let access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");
    let region = Region::new("us-east-1"); // Replace with your desired AWS region

    // Configure AWS S3 client
    let config = Config::builder()
        .credentials_provider(
            aws_sdk_s3::AwsCredentialsProvider::default().set_keys(access_key, secret_key),
        )
        .region(region)
        .build();
    let client = Client::from_conf(config);

    // Start TCP listener on port 8000
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("Server listening on port 8000");

    // Handle incoming connections
    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, &client).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

// Handle an incoming connection
async fn handle_connection(
    socket: tokio::net::TcpStream,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    let mut filename = String::new();
    let mut file_contents = Vec::new();

    // Read the file name and contents from the incoming request
    while let Ok(n) = socket.read_to_end(&mut buf).await {
        if n == 0 {
            break;
        }

        // Parse the file name and contents from the request
        let request = String::from_utf8_lossy(&buf);
        if let Some(index) = request.find("filename=\"") {
            let filename_start = index + 10;
            let filename_end = request[filename_start..].find("\"").unwrap() + filename_start;
            filename = request[filename_start..filename_end].to_string();
        }
        if let Some(index) = request.find("\r\n\r\n") {
            file_contents = buf[index + 4..].to_vec();
            break;
        }

        buf.clear();
    }

    // Upload file to S3 bucket
    let request = aws_sdk_s3::input::PutObjectRequest::builder()
        .bucket("your-bucket-name") // Replace with your S3 bucket name
        .key(filename)
        .body(ByteStream::from(file_contents))
        .build();

    match client.put_object(request).await {
        Ok(_) => {
            println!("File uploaded to S3 successfully");
            socket
                .write_all(b"HTTP/1.1 200 OK\r\n\r\nFile uploaded to S3 successfully")
                .await?;
        }
        Err(e) => {
            eprintln!("Failed to upload file to S3: {}", e);
            socket
                .write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\nInternal server error")
                .await?;
        }
    }

    Ok(())
}

// curl -X POST -F "file=@/path/to/file" http://localhost:8080/upload
