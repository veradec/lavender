#![allow(unused)]
use anyhow::Result;
use markdown;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

macro_rules! string_to_bool {
    ($str:expr) => {
        match &$str.to_lowercase().as_str()[0..4] {
            "true" => true,
            _ => false,
        }
    };
}

struct HTMLFile {
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
    let (mut date, mut title, mut desc) = (String::new(), String::new(), String::new());
    let mut toc: bool = true;
    let mut header: bool = true;
    for line in lines {
        let meta_data = line.split('=').collect::<Vec<&str>>();
        match meta_data[0].trim_start().trim_end() {
            "date" => date = meta_data[1].to_string(),
            "title" => title = meta_data[1].to_string(),
            "description" => desc = meta_data[1].to_string(),
            "toc" => toc = string_to_bool!(meta_data[1].to_string().trim_start()),
            "header" => header = string_to_bool!(meta_data[1].to_string().trim_start()),
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

    let (date, title, description, toc, header) = meta_data_extractor(meta_data);

    let mut options = markdown::Options::gfm();
    options.compile.allow_dangerous_html = true;

    let mut html = markdown::to_html_with_options(&content[end + 7..], &options).unwrap();

    let mut file_name = std::path::Path::new(&path)
        .display()
        .to_string()
        .replacen("content", "public", 1);
    if file_name != "public/index.md" {
        file_name = file_name[0..(file_name.len() - 3)].to_string() + "/index.html";

        html = fs::read_to_string("templates/post.html")
            .unwrap()
            .replace("{{content}}", &html)
            .replace("{{title}}", &title)
            .replace("{{description}}", &description);
    } else {
        file_name = "public/index.html".to_string();
        html = fs::read_to_string("templates/index.html")
            .unwrap()
            .replace("{{content}}", &html)
            .replace("{{title}}", &title)
            .replace("{{description}}", &description);
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

fn write_html_from_md(path: &Path) {
    let contents_read = fs::read_to_string(path).unwrap();
    let f: HTMLFile = markdown_to_html(path.display().to_string(), contents_read);
    println!("{}", f.file_name);
    let path = Path::new(&f.file_name);
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

fn generate_lists(dir: PathBuf) -> std::io::Result<()> {
    let mut dir_names: Vec<String> = vec![];
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.path().to_string_lossy().ends_with("index.html") {
            continue;
        } else {
            generate_lists(entry.path());
        }
        dir_names.push(entry.path().to_string_lossy().to_string());
    }

    // there is a index.html, then skip
    // read all the directories inside the directory and save it to index.html
    if dir_names.len() > 0 {
        if &dir.to_string_lossy() != "public/" {
            // println!("{:?} - {:?}", &dir.to_string_lossy(), dir_names);
            vec2html(dir.to_string_lossy().to_string(), dir_names);
        }
    }
    Ok(())
}

fn vec2html(mut path: String, dir_names: Vec<String>) {
    path = path + "/index.html";
    let mut content = String::new();
    println!("File: {}", path);
    content += "<ul>\n";

    for dir in dir_names {
        let names = dir.split("/").collect::<Vec<&str>>();
        content += &"<li><a href={dir}>{name}</a></li>\n"
            .replace("{dir}", &dir[6..])
            .replace("{name}", &names[names.len() - 1].replace("-", " "));
    }
    content += "</ul>";
    content = fs::read_to_string("templates/list.html")
        .unwrap()
        .replace("{{content}}", &content);
    fs::write(path, content);
}

fn main() -> Result<()> {
    for file in WalkDir::new("content/").into_iter().filter_map(|e| e.ok()) {
        let file_name = file.file_name().to_string_lossy();
        if file_name.ends_with(".md") {
            write_html_from_md(file.path());
        }
    }

    generate_lists("public/".into());
    // copy static folder to public folder
    copy_dir(Path::new("static"), Path::new("public"));

    Ok(())
}
