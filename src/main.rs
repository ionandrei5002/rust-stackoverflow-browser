extern crate curl;
extern crate select;

use std::str;
use std::string;
use std::collections::HashMap;
use curl::easy::Easy;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Attr;

fn main() {
    let mut response = Vec::new();
    let mut http = Easy::new();
    let root = "https://stackoverflow.com".to_string();
    let url = "".to_string();
    let search = url + &root + "/search?q=rust";

    http.url(&search).unwrap();
    let mut list = HashMap::new();
    {
        let mut transfer = http.transfer();
        transfer.header_function(|header: &[u8]| {
            let value: String = str::from_utf8(header).unwrap().to_string();
            let parts: Vec<&str> = value.trim().split(':').collect();
            if parts.len() == 1 {
                list.insert(parts[0].to_string(), "".to_string());
            } else {
                list.insert(parts[0].to_string(), parts[1].trim().to_string());
            }
            true
        }).unwrap();
        transfer.perform().unwrap();
    }

    println!("{:?}", list);
    
    {
        let mut transfer = http.transfer();
        transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let result = String::from_utf8(response).unwrap();
    // println!("{}", result);

    // Document::from_read(result.as_bytes())
    //     .unwrap()
    //     .find(Name("a"))
    //     .filter_map(|n| {
    //         n.attr("href")
    //     })
    //     .for_each(|x| {
    //         println!("{}", x)
    //     });

    Document::from_read(result.as_bytes())
        .unwrap()
        .find(Attr("class", "question-hyperlink"))
        .filter_map(|n| {
            n.attr("href")
        })
        .for_each(|x| {
            println!("{}", x)
        });
}
