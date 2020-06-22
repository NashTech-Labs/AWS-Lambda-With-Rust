use lambda_runtime::{error::HandlerError, lambda};
use std::error::Error;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<String>,
}

#[derive(Deserialize, Clone)]
struct QueryString {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Body {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    body: String,
}

impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            body,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda_runtime::Context) -> Result<CustomOutput, HandlerError> {
    if let Some(q) = e.query_string_parameters {
        if let Some(first_name) = q.first_name {
            return match first_name.as_ref() {
                "" => Ok(CustomOutput::new(format!(
                    "Hello! Welcome to the rust program in Lambda Function with empty parameter"
                ))),
                "error" => Err(c.new_error("Empty first name")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello {}!!! Welcome to the rust program in Lambda Function. ",
                    first_name
                ))),
            };
        }
    }

    if let Some(b) = e.body {
        let parsed_body: Result<Body, serde_json::Error> = serde_json::from_str(&b);
        if let Ok(result) = parsed_body {
            return match result.first_name.as_ref().map(|s| &s[..]) {
                Some("") => Ok(CustomOutput::new(format!(
                    "Hello! Welcome to the rust program in Lambda Function with empty body parameter"
                ))),
                Some("error") => Err(c.new_error("Empty first name in body parameter")),
                _ => Ok(CustomOutput::new(format!(
                    "Hello {}! Welcome to the rust program in Lambda Function",
                    result.first_name.unwrap_or("".to_owned())
                ))),
            };
        }
    }

    Ok(CustomOutput {
        body: format!("Hello! Welcome to the rust program in Lambda Function. Please add parameters."),
    })
}
