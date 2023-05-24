#[macro_use]
extern crate rocket;

use rocket::data::{ByteUnit, FromDataSimple};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::tokio::fs::{self, File};
use rocket::tokio::io::AsyncWriteExt;
use rocket::Data;
use rocket::Request;

#[derive(Debug)]
struct UploadedFile(File);

#[rocket::async_trait]
impl<'r> FromDataSimple<'r> for UploadedFile {
    type Error = std::io::Error;

    async fn from_data(
        _req: &'r Request<'_>,
        data: Data<'r>,
    ) -> rocket::data::Outcome<'r, 'static> {
        let file = match data.open(ByteUnit::default()).into_inner().await {
            Ok(file) => file,
            Err(err) => return rocket::data::Outcome::Failure((Status::InternalServerError, err)),
        };

        rocket::data::Outcome::Success(UploadedFile(file))
    }
}

#[post("/upload", data = "<file>")]
async fn upload(file: UploadedFile) -> Custom<()> {
    let file_name = file.0.filename().unwrap();
    let file_path = format!("uploads/{}", file_name);

    let mut dest = match fs::File::create(&file_path).await {
        Ok(file) => file,
        Err(err) => {
            return Custom(
                Status::InternalServerError,
                format!("Failed to create file: {}", err),
            )
        }
    };

    if let Err(err) = file.0.stream_to(&mut dest).await {
        return Custom(
            Status::InternalServerError,
            format!("Failed to save file: {}", err),
        );
    }

    Custom(Status::Ok, ())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}

// To run the server on a different port, modify the rocket() function in the code and specify the desired port.
// Here's an example of how to change the port to 8080:
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![upload]).port(8080)
// }
