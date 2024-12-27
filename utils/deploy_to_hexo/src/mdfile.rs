use comrak::nodes::{NodeValue, ValidationError};
use comrak::{format_commonmark, format_html, parse_document, Arena, Options};
use core::fmt;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::io::{Read, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct MdFile {
    md: PathBuf,
    imgs: Vec<PathBuf>,
    parent: PathBuf,
    t_fname: String,
    img_dir: String,
}
impl MdFile {
    pub fn new(md_file_entry: &DirEntry) -> MdFile {
        let parent_path = md_file_entry.path().parent().unwrap_or(Path::new(""));
        let grand_parent_path = parent_path.parent().unwrap_or(Path::new(""));
        let mut img_paths: Vec<PathBuf> = vec![];
        let imges_dir = format!("{}/images", parent_path.to_str().unwrap());
        // println!("{:?}", imges_dir);
        for img_entry in WalkDir::new(imges_dir).into_iter().filter_map(|e| e.ok()) {
            if img_entry.file_type().is_file() {
                img_paths.push(img_entry.into_path());
            }
        }
        let _t_fname: String = format!(
            "{}_{}",
            &grand_parent_path
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap(),
            &md_file_entry.file_name().to_str().unwrap()
        )
        .to_string();
        let img_dir = String::from(_t_fname.get(0.._t_fname.len() - 3).unwrap());
        // println!("{:?}", img_paths);
        MdFile {
            md: md_file_entry.clone().into_path(),
            parent: parent_path.to_path_buf(),
            imgs: img_paths.clone(),
            t_fname: _t_fname,
            img_dir: img_dir,
        }
    }

    pub fn get_content(&self) -> String {
        // let markdown = "Here is an image: ![alt text](image.jpg)";
        let mut md_file = File::open(&self.md).unwrap();
        let mut markdown = String::new();
        md_file.read_to_string(&mut markdown).unwrap();
        // let arena = Arena::new();
        // let root = parse_document(&arena, &markdown.as_str(), &Options::default());
        // let mut new_img_path = String::from("new_img_path.png");
        // for node in root.descendants() {
        // if let NodeValue::Image(ref mut img) = node.data.borrow_mut().value {
        // let old_url = img.url.clone();
        // let picturename = Path::new(old_url.as_str())
        // .file_name()
        // .unwrap()
        // .to_str()
        // .unwrap();
        // img.url = String::from(format!("/images/{}/{}", self.img_dir, picturename));
        // }
        // }

        // let mut md_text = vec![];
        // format_commonmark(root, &Options::default(), &mut md_text).unwrap();
        // String::from_utf8(md_text).unwrap()
        let mut update_str_vec: Vec<(String, String)> = vec![];
        let re = Regex::new(r"!\[.*\]\(.*\)").unwrap();
        for cap in re.captures_iter(&markdown) {
            if let Some(image_md) = cap.get(0) {
                let image_md_str = String::from(image_md.as_str());
                let image_url_str_start = image_md_str.find("(").unwrap_or(0) + 1;
                let image_url_str_end = image_md_str.find(")").unwrap_or(0);
                let image_path = image_md_str
                    .get(image_url_str_start..image_url_str_end)
                    .unwrap();
                let image_name = Path::new(image_path).file_name().unwrap().to_str().unwrap();
                // println!("图片url: {:?} 图片名: {:?}", image_path, image_name);
                update_str_vec.push((
                    String::from(image_path),
                    String::from(format!("/images/{}/{}", self.img_dir, image_name)),
                ));
            }
        }

        for update_str in update_str_vec {
            markdown = markdown.replace(update_str.0.as_str(), update_str.1.as_str());
        }

        markdown
    }

    pub fn deploy(&self, deploy_path: &Path) {
        let deploy_image_path = deploy_path.join(format!("source/images/{}", self.img_dir));
        let deploy_md_path = deploy_path.join(format!("source/_posts/{}", self.t_fname));
        let content = self.get_content();
        let mut md_file = File::create(deploy_md_path).unwrap();
        md_file.write_all(content.as_bytes()).unwrap();
        if deploy_image_path.exists() {
            fs::remove_dir_all(&deploy_image_path).unwrap();
        }
        fs::create_dir_all(&deploy_image_path).unwrap();
        for img_path in self.imgs.iter() {
            fs::copy(
                img_path.to_str().unwrap(),
                deploy_image_path
                    .clone()
                    .join(img_path.file_name().unwrap()),
            )
            .unwrap();
        }
    }
}

impl fmt::Display for MdFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MdFile {{ md: {:?}, imgs: {:?}, parent: {:?}, t_fname: {} }}",
            self.md, self.imgs, self.parent, self.t_fname
        )
    }
}
