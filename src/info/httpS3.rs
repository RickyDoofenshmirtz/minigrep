use rocket::{post, routes, Rocket};
use rocket::data::ByteUnit;
use rocket::data::Data;
use rocket::http::Status;
use rocket::response::status;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3, S3Client};
use std::io::Read;

#[post("/upload", data = "<data>")]
async fn upload(data: Data<'_>) -> status::Custom<String> {
    // Specify your AWS credentials and region
    let region = Region::default();

    // Create an S3 client
    let client = S3Client::new(region);

    // Specify the S3 bucket name and object key
    let bucket_name = "your-bucket-name";
    let object_key = "your-object-key";

    // Read the data from the request body
    let mut buffer = Vec::new();
    if let Err(err) = data.open(ByteUnit::default()).read_to_end(&mut buffer) {
        return status::Custom(Status::InternalServerError, format!("Error reading request body: {:?}", err));
    }

    // Create the PutObjectRequest
    let request = PutObjectRequest {
        bucket: bucket_name.to_string(),
        key: object_key.to_string(),
        body: Some(buffer.into()),
        ..Default::default()
    };

    // Upload the object to S3
    match client.put_object(request).await {
        Ok(_) => status::Custom(Status::Ok, "Object uploaded successfully".to_string()),
        Err(err) => status::Custom(Status::InternalServerError, format!("Error uploading object: {:?}", err)),
    }
}

#[launch]
fn rocket() -> Rocket {
    rocket::build().mount("/", routes![upload])
}

fn main() {
    rocket().launch();
}


// curl -X POST -H "Content-Type: application/octet-stream" --data-binary @/path/to/file.ext http://localhost:8000/upload
