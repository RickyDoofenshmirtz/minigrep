use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::response::status;
use rocket::Data;
use rocket::State;
use rocket_contrib::json::JsonValue;
use rusoto_core::Region;
use rusoto_credential::DefaultCredentialsProvider;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::io::Read;

#[post("/upload", data = "<data>")]
fn upload_file(data: Data, s3_client: State<S3Client>) -> status::Custom<JsonValue> {
    let mut buffer = Vec::new();
    if let Err(_) = data.open().read_to_end(&mut buffer) {
        return status::Custom(
            Status::InternalServerError,
            json!({ "error": "Failed to read file data" }),
        );
    }

    // Customize the S3 bucket and object key according to your setup
    let bucket = "your-bucket-name";
    let key = "uploaded-file.txt";

    let request = PutObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        body: Some(buffer.into()),
        ..Default::default()
    };

    match s3_client.put_object(request).sync() {
        Ok(_) => status::Custom(
            Status::Ok,
            json!({ "message": "File uploaded successfully" }),
        ),
        Err(e) => {
            eprintln!("S3 upload error: {:?}", e);
            status::Custom(
                Status::InternalServerError,
                json!({ "error": "Failed to upload file to S3" }),
            )
        }
    }
}

fn rocket() -> rocket::Rocket {
    let s3_client = S3Client::new_with(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);

    rocket::ignite()
        .manage(s3_client)
        .mount("/", routes![upload_file])
}

fn main() {
    rocket().launch();
}

// curl -X POST -F "file=@path/to/file.jpg" http://localhost:8000/upload

// [dependencies]
// rocket = "0.5.0-rc.1"
// rocket_contrib = "0.5.0-rc.1"
// rusoto_core = "0.44"
// rusoto_credential = "0.44"
// rusoto_s3 = "0.44"
