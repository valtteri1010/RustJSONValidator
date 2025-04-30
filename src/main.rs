use reqwest::blocking::get;
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq)]
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

fn expected_response() -> Todo {
    Todo {
        userId: 1,
        id: 1,
        title: "delectus aut autem".to_string(),
        completed: false,
    }
}

fn check_response() -> Result<bool, Box<dyn std::error::Error>> {
    let resp = get("https://jsonplaceholder.typicode.com/todos/1")?;
    if resp.status() != 200 {
        println!("Failed: Expected status 200, got {}", resp.status());
        return Ok(false);
    }

    let actual: Todo = resp.json()?;
    let expected = expected_response();

    if actual == expected {
        println!("Test passed");
        Ok(true)
    } else {
        println!("Test failed");
        if actual.userId != expected.userId {
            println!("Mismatch on userId: expected {}, got {}", expected.userId, actual.userId);
        }
        if actual.id != expected.id {
            println!("Mismatch on id: expected {}, got {}", expected.id, actual.id);
        }
        if actual.title != expected.title {
            println!("Mismatch on title: expected {:?}, got {:?}", expected.title, actual.title);
        }
        if actual.completed != expected.completed {
            println!("Mismatch on completed: expected {}, got {}", expected.completed, actual.completed);
        }
        Ok(false)
    }
}

fn main() {
    if let Err(e) = check_response() {
        eprintln!("Error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response() {
        let result = check_response().expect("HTTP request failed");
        assert!(result, "API response did not match expected values");
    }
}
