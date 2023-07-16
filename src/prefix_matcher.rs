// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

const SPLITTER: &str = "/";

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {

    if prefix == request_path {
        return true
    }

    let pref_vec: Vec<String> = prefix.split(SPLITTER)
    .filter(|p| !p.is_empty() )
    .map(|p| p.to_string())
    .collect();

    let req_path_vec: Vec<String> = request_path.split(SPLITTER)
    .filter(|p| !p.is_empty() )
    .map(|p| p.to_string())
    .collect();

    if pref_vec.len() > req_path_vec.len() {
        return false;
    }

    let mut is_matches = true;
    let mut prev_pref_value: Option<&String> = None;

    for (i, path_chunc) in req_path_vec.into_iter().enumerate() {
        let current_prefix_container = pref_vec.get(i);
        if current_prefix_container == None {
            break;
        }

        let current_prefix = current_prefix_container.unwrap();

        match path_chunc {
            s if &s == current_prefix => {
                prev_pref_value = Some(current_prefix);
                continue;
            },
            s if current_prefix == &String::from("*") => {
                continue;
            }
            s if prev_pref_value == Some(&String::from("*")) => {
                continue;
            }
            _  => {
                is_matches = false;
                break;
            }
        }
    }

    is_matches
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
 fn main() {}
