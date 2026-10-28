#![allow(unused)]
use anyhow::Result;
use markdown;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

struct File {
    file_name: String,
    date: String,
    toc: bool,
    header: bool,
    title: String,
    description: String,
    content: String,
}

fn meta_data_extractor(content: &str) -> (String, String, String, bool, bool) {
    let lines = content.lines();
    let mut date: String = "".to_string();
    let mut title: String = "".to_string();
    let mut desc: String = "".to_string();
    let mut toc: bool = true;
    let mut header: bool = true;
    for line in lines {
        let meta_data = line.split('=').collect::<Vec<&str>>();
        match meta_data[0].trim_start().trim_end() {
            "date" => date = meta_data[1].to_string(),
            "title" => title = meta_data[1].to_string(),
            "description" => desc = meta_data[1].to_string(),
            // "toc" => println!("{}", meta_data[1].to_string().trim_start().trim_end()),
            // "header" => println!("{}", meta_data[1].to_string().trim_start().trim_end()),
            "" => (),
            &_ => println!(
                "Header not found |{}|",
                meta_data[0].trim_start().trim_end()
            ),
        }
    }
    return (date, title, desc, toc, header);
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

    let mut date: String;
    let mut title: String;
    let mut description: String;
    let mut toc: bool;
    let mut header: bool;
    (date, title, description, toc, header) = meta_data_extractor(meta_data);
    // println!("{meta_data}\n\n");

    let html = markdown::to_html(&content[end + 7..]);
    let file_name = std::path::Path::new(&path).display().to_string();

    return File {
        file_name: file_name,
        date: date,
        title: title,
        description: description,
        content: html,
        toc: toc,
        header: header,
    };
}

fn get_html_from_file(path: &Path) -> File {
    // println!("{}", path.display());
    let contents_read = fs::read_to_string(path).unwrap();
    let f: File = markdown_to_html(path.display().to_string(), contents_read);
    println!("file_name: {}\ncontent:\n{}\n", f.file_name, f.content);
    println!("-------------------------------------");
    f
}

fn main() -> Result<()> {
    // let contents_path = std::path::Path::new("./content/index.md");
    // let path = "./content/";
    let content_path = Path::new("content/");

    for file in WalkDir::new("content/").into_iter().filter_map(|e| e.ok()) {
        let file_name = file.file_name().to_string_lossy();
        if file_name.ends_with(".md") {
            // println!("{}", file.path().display());
            get_html_from_file(file.path());
        }
    }

    // println!("File parent_directory: {}", path);

    // let contents_read = fs::read_to_string(path).unwrap();
    // let f: File = markdown_to_html(path.to_string(), contents_read);
    // println!(
    //     "file_name: {}\npath:{}\ncontent:\n{}\n",
    //     f.file_name, f.parent_directory, f.content
    // );
    Ok(())
}
