use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

#[tokio::main]
async fn main() {
    // AWS credentials and region
    let region = Region::default();

    // Create an S3 client
    let client = S3Client::new(region);

    // S3 bucket name and object key
    let bucket_name = "your-bucket-name";
    let object_key = "your-object-key";

    // Read the data from a file or any other source
    let data = b"Hello, world!";

    // Create the PutObjectRequest
    let request = PutObjectRequest {
        bucket: bucket_name.to_string(),
        key: object_key.to_string(),
        body: Some(data.to_vec().into()),
        ..Default::default()
    };

    // Upload the object to S3
    match client.put_object(request).await {
        Ok(_) => println!("Object uploaded successfully"),
        Err(err) => eprintln!("Error uploading object: {:?}", err),
    }
}

// [dependencies]
// rusoto_core = "0.43"
// rusoto_s3 = "0.43"
// tokio = { version = "1", features = ["full"] }
