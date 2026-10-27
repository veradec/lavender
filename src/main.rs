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

fn markdown_to_html(path: String, content: String) -> File {
    let mut end: usize = 0;
    if &content[0..3] != "+++" {
        println!("It doesnt pass");
    }

    let end = content[3..].find("+++").unwrap();

    // if Some(end) == Some(None) {
    //     println!("It doesnt pass");
    // }

    let meta_data = &content[3..end];

    println!("{meta_data}\n\n");

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
    println!(
        "file_name: {}\npath:{}\ncontent:\n{}",
        f.file_name, f.parent_directory, f.content
    );
}
