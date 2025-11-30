use rocket::State;
use rocket::{http::Status, response::content::RawHtml};
use rocket::tokio;

pub struct SharedComponents {
    pub style: String,
}

pub fn stylize_html(style: &State<SharedComponents>, input: &mut String, document_title: &String) {
    let details = Details {
        title: &document_title,
        content: &input
    };
    *input = apply_string_details(details, &style.style);
}


pub async fn try_sanitize_path(unsafe_path: &std::path::PathBuf, expected_folder: &std::path::PathBuf) -> Result<(), (Status, Option<RawHtml<String>>)> {
    // get true paths for both folders
    let true_expected_folder = match tokio::fs::canonicalize(&expected_folder).await {
        Ok(r) => r,
        Err(_) => return Err((Status::InternalServerError, None)),
    };

    let cannon = match tokio::fs::canonicalize(unsafe_path).await {
        Ok(r) => r,
        Err(_) => return Err((Status::NotFound, None))
    };
    
    // see if the cannon (dangerous) folder isn't a child of the allowed "sanbox" folder, 
    // if not, yell at the sender.
    if !cannon.starts_with(true_expected_folder) {
        println!("ALERT: someone might be attempting directory traversal");
        return Err((Status::Forbidden, Some(RawHtml("YOU SHALL NOT PASS! invalid directories to the server".to_string()))));
    }
    
    // otherwise, everything is good
    Ok(())
}

/// details for an html template
pub struct Details<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

impl Details<'_> {
    /// return a value of a details struct's key based off of a string
    pub fn fetch_detail_from_str<'a>(&'a self, key: &str) -> &'a str {
        // sorry for the lack of efficiency a HashMap could bring. I find it easier for people
        // modifying this script to use a struct though.
        match key {
            "title" => {return self.title;},
            "content" => {return self.content;}
            _ => {print!("unknown key \"{}\"", key);}
        }
        ""
    }
}

/// apply details to a string, used to put title and other document details on an html response
pub fn apply_string_details(details: Details<'_>, original: &String) -> String  {
    let mut buff = String::new();
    let chars: Vec<char> = original.chars().collect();
    let mut in_field: bool = false;
    let mut arg_buffer = String::new();
    let mut close_on_next = false; 

    for (i, n) in chars.clone().into_iter().enumerate() {
        match in_field {
            true => {
                if close_on_next {
                    // reset variables
                    in_field = false;
                    close_on_next = false;
                    buff.push_str(details.fetch_detail_from_str(&arg_buffer)); // add the string
                    arg_buffer.clear();
                    continue; 
                }
                // because when we continue from an opening, there are two of `{`
                match n {
                    '{' => {continue;},
                    '}' => {close_on_next = true; continue;},
                    ' ' => continue,
                    _ => {arg_buffer.push(n); continue;}
                }
            },
            false => {
                if n == '{' && chars[i + 1] == '{' {
                    in_field = true;
                    continue;       
                }
                buff.push(n);
            }
        }
    }
    
    buff
}

#[test]
fn test_string_details() {
    let details = Details { title: "some cool title", content: "cool contents fr fr" };
    let test_string = String::from("title: {{ title }}, contents: {{ contents }} {this should be unaffected}");
    let result = apply_string_details(details, &test_string);
    assert_eq!(result, "title: some cool title, contents: cool contents fr fr {this should be unaffected}")
}
