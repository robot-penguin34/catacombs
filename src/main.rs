use std::ffi::OsString;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::tokio;
use rocket::{response::content::RawHtml};
use rocket::State;
use utilities::{stylize_html, SharedComponents};

#[macro_use] extern crate rocket;
mod utilities;


#[get("/secret")]
async fn rickroll() -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!("https://youtu.be/klfT41uZniI?si=WfZiLxyrtoPYZHc0"))
}

#[get("/")]
async fn index(style: &State<SharedComponents>) -> Result<ResponseVariant, (Status, Option<RawHtml<String>>)> {
    // rather than redirect, just make it serve that function to make the url cleaner.
    serve_public_file(style, "index.md".into()).await
}

#[derive(Responder)]
enum ResponseVariant {
    HTML(RawHtml<String>),
    OTHER(NamedFile),
}

//TODO: see if this is vulnerable to directory traversal attacks
#[get("/<file..>")]
async fn serve_public_file(style: &State<SharedComponents>, file: PathBuf) -> Result<ResponseVariant, (Status, Option<RawHtml<String>>)> {
    // public_folder is to see if the requested path is outside of the allowed public folder, if it is something sketchy is going on.

    let public_folder = match tokio::fs::canonicalize(Path::new("public/")).await {
        Ok(res) => res,
        Err(_) => {
            println!("Critical Error: unable to locate the public/ folder");
            return Err((Status::InternalServerError, None));
        }
    };
    let requested_path = Path::new("public/").join(file);
    
    // try to prevent traversal attacks
    utilities::try_sanitize_path(&requested_path, &public_folder).await?;
    
    // if the file **isn't** html just return it as a normal file (not rawhtml)
    let extension = requested_path.extension()
        .unwrap_or(&OsString::from(""))
        .to_os_string()
        .into_string()
        .unwrap_or(String::from("")); // sorry to people who don't like using unwrap_or too many
                                      // times
    
    let title = requested_path.file_name()
        .unwrap_or(&OsString::from("title error"))
        .to_os_string()
        .into_string()
        .unwrap_or(String::from("title error"));


    match extension.as_str() {
          "html" => {
            let res = match tokio::fs::read_to_string(&requested_path).await {
                Ok(mut r) => {
                    stylize_html(style, &mut r, &title); 
                    r
                },
                Err(_) => {return Err((Status::NotFound, None))}
            };

            return Ok(ResponseVariant::HTML(RawHtml(res)));

        },

        "" => {return Err((Status::BadRequest, None))},
        "md" => {
            let res = match tokio::fs::read_to_string(&requested_path).await {
                Ok(mut r) => {
                    r = markdown::to_html(r.as_str());
                    stylize_html(style, &mut r, &title);
                    r
                },
                Err(_) => {return Err((Status::NotFound, None))}
            };
            return Ok(ResponseVariant::HTML(RawHtml(res)));
        },

        _ => {
            let resource = match NamedFile::open(&requested_path).await {
                Ok(r) => r,
                Err(_) => return Err((Status::NotFound, None)),
            };

            return Ok(ResponseVariant::OTHER(resource));

        },
    }
    
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let theme_fallback = "<h1>Theme error. theme should be called \"style.html\" and placed in the resources/ folder</h1>\n{{ content }}".to_string();
    let shared: SharedComponents = SharedComponents {
        style: tokio::fs::read_to_string(Path::new("resources/").join("style.html")).await.unwrap_or(theme_fallback.clone()),
    };
    let _rocket = rocket::build()
        .mount("/", routes![
            index,
            rickroll,
            serve_public_file,
        ])
        .manage(shared)
        .launch()
        .await?;

    Ok(())
}
