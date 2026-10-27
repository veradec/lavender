#![allow(unused)]
use markdown;
use std::env;
use std::fs;
use std::path::PathBuf;

struct File {
    file_name: String,
    parent_directory: String,
    date: Option<String>,
    title: Option<String>,
    description: Option<String>,
    content: String,
}

fn meta_data_extractor(content: &str) -> (String, String, String) {
    let lines = content.lines();
    let mut date: String = "".to_string();
    let mut title: String = "".to_string();
    let mut desc: String = "".to_string();

    for line in lines {
        let meta_data = line.split('=').collect::<Vec<&str>>();
        match meta_data[0] {
            "date" => date = meta_data[1].to_string(),
            "title" => title = meta_data[1].to_string(),
            "description" => desc = meta_data[1].to_string(),
            &_ => println!(""),
        }
    }

    return (date, title, desc);
}

fn markdown_to_html(path: String, content: String) -> File {
    let mut end: usize = 0;
    if &content[0..3] != "+++" {
        println!("It doesnt pass");
    }

    let end_wrapped = content[3..].find("+++");

    if Some(end_wrapped) == Some(None) {
        println!("It doesnt pass");
    }

    let end = end_wrapped.unwrap();
    let meta_data = &content[3..end + 2];

    let date: String;
    let title: String;
    let description: String;

    meta_data_extractor(meta_data);
    // println!("{meta_data}\n\n");

    let html = markdown::to_html(&content[end + 7..]);
    let file_name = std::path::Path::new(&path).file_name().unwrap();
    return File {
        file_name: file_name.to_str().unwrap().to_string(),
        parent_directory: path,
        date: None,
        title: None,
        description: None,
        content: html,
    };
}

fn main() {
    // let contents_path = std::path::Path::new("./content/index.md");
    let path = "./content/index.md";
    // let path = std::env::current_dir()
    //     .unwrap()
    //     .into_os_string()
    //     .into_string();
    // println!("File parent_directory: {}", path);

    let contents_read = fs::read_to_string(path).unwrap();
    let f: File = markdown_to_html(path.to_string(), contents_read);
    // println!(
    //     "file_name: {}\npath:{}\ncontent:\n{}",
    //     f.file_name, f.parent_directory, f.content
    // );
}
