use mdfile::MdFile;
use walkdir::DirEntry;
use walkdir::WalkDir;
mod mdfile;
use std::path::Path;

fn get_md_files(path: &Path, exclude: Vec<&str>) -> Vec<DirEntry> {
    let mut res: Vec<DirEntry> = vec![];
    for entry in WalkDir::new(path)
        .min_depth(0)
        .into_iter()
        .filter_entry(|e| !exclude.contains(&e.file_name().to_str().unwrap()))
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() && entry.file_name().to_str().unwrap().ends_with(".md") {
            res.push(entry);
        }
    }
    res
}

fn main() {
    let path = Path::new("/home/ziyang/works/Kernel-Fuzzing-Tutorial");
    let target_path = Path::new("/home/ziyang/self-blog");

    let exclude = vec![".git", "README.md", "paper", "src", "utils"];
    let md_files = get_md_files(path, exclude);
    let mut md_objs: Vec<MdFile> = vec![];
    for mde in md_files.iter() {
        md_objs.push(MdFile::new(mde));
    }
    for md in md_objs.iter() {
        println!("{}", md);
        md.deploy(target_path);
    }
}
