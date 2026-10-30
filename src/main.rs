#![allow(unused)]
use anyhow::Result;
use markdown;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

struct HTMLFile {
    file_name: String,
    date: String,
    toc: bool,
    header: bool,
    title: String,
    description: String,
    content: String,
}

fn string_to_bool(str: &str) -> bool {
    let sliced = &str[0..4];
    if sliced == "true" {
        return true;
    }
    false
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
            "toc" => toc = string_to_bool(meta_data[1].to_string().trim_start()),
            "header" => header = string_to_bool(meta_data[1].to_string().trim_start()),
            "" => (),
            &_ => println!(
                "Header not found |{}|",
                meta_data[0].trim_start().trim_end()
            ),
        }
    }
    return (date, title, desc, toc, header);
}

fn markdown_to_html(path: String, content: String) -> HTMLFile {
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

    let mut html = markdown::to_html(&content[end + 7..]);

    let mut file_name = std::path::Path::new(&path)
        .display()
        .to_string()
        .replacen("content", "public", 1);
    if file_name != "public/index.md" {
        file_name = file_name[0..(file_name.len() - 3)].to_string() + "/index.html";

        html = fs::read_to_string("templates/post.html")
            .unwrap()
            .replace("{{content}}", &html);
    } else {
        file_name = "public/index.html".to_string();
        html = fs::read_to_string("templates/index.html")
            .unwrap()
            .replace("{{content}}", &html);
    }

    return HTMLFile {
        file_name,
        date,
        title,
        description,
        content: html,
        toc,
        header,
    };
}

fn get_html_from_file(path: &Path) -> HTMLFile {
    let contents_read = fs::read_to_string(path).unwrap();
    let f: HTMLFile = markdown_to_html(path.display().to_string(), contents_read);
    f
}

fn write_content_to_file(f: HTMLFile) {
    let file_path = f.file_name;
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent);
    }
    fs::write(path, f.content);
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // copy static folder to public folder
    copy_dir(Path::new("static"), Path::new("public"));
    //////

    for file in WalkDir::new("content/").into_iter().filter_map(|e| e.ok()) {
        let file_name = file.file_name().to_string_lossy();
        if file_name.ends_with(".md") {
            write_content_to_file(get_html_from_file(file.path()));
        }
    }
    Ok(())
}
