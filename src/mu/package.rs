use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::fs::{self, OpenOptions};

use toml;
use super::error::Error;

#[derive(Debug)]
pub struct FileTree {
    pub directories: HashMap<PathBuf, Vec<PathBuf>>,
}

impl FileTree {
    fn empty() -> FileTree {
        FileTree {
            directories: HashMap::new(),
        }
    }

    fn from_root(path: &Path) -> io::Result<FileTree> {
        fn recursive_from_root(path: &Path,
                               filetree: &mut FileTree,
                               files: &mut Vec<PathBuf>) -> io::Result<()> {
            for entry in try!(fs::read_dir(path)) {
                let entry = try!(entry);
                let metadata = try!(entry.metadata());

                if metadata.is_dir() {
                    let mut children = Vec::new();
                    try!(recursive_from_root(&entry.path(), filetree, &mut children));
                    if children.len() > 0 {
                        filetree.directories.insert(entry.path(), children);
                        files.push(entry.path());
                    }
                } else {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "v" {
                            files.push(path.to_owned())
                        }
                    }
                }
            }

            Ok(())
        }

        let mut filetree = FileTree::empty();
        let mut files = Vec::new();

        try!(recursive_from_root(path, &mut filetree, &mut files));

        // Add the root path and top-level files.
        filetree.directories.insert(path.to_owned(), files);

        Ok(filetree)
    }
}


#[derive(Debug)]
pub struct Package {
    pub root: PathBuf,
    name: String,
    version: String,
    authors: Option<Vec<String>>,
    dependencies: Vec<(String, String)>,
    pub filetree: FileTree,
}

impl Package {
    pub fn from_root(root: &Path) -> Result<Package, Error> {
        let mut file = try!(OpenOptions::new()
                            .read(true)
                            .open(root.join("Gallus.toml")));
        let mut file_contents = String::new();
        try!(file.read_to_string(&mut file_contents));
        match toml::Parser::new(&file_contents[..]).parse() {
            None => panic!("couldn't parse toml file {:?}", root.join("Gallus.toml")),
            Some(v) => Package::from_value(root, v),
        }
    }

    pub fn from_value(root: &Path, value: toml::Table) -> Result<Package, Error> {
        use toml::Value;

        fn ensure_string(table: &toml::Table, key: &str) -> Result<String, Error> {
            let value = table.get(key);

            match value {
                Some(&Value::String(ref s)) => Ok(s.clone()),
                Some(_) => Err(Error::TypeMismatch),
                None => Err(Error::MissingKey(key.to_string())),
            }
        }

        let (name, version, authors) = match value.get("package") {
            Some(&Value::Table(ref table)) => {
                let name = try!(ensure_string(table, "name"));
                let version = try!(ensure_string(table, "version"));
                let authors = None; // TODO
                (name, version, authors)
            }
            _ => panic!("err"),
        };

        let deps = value.get("dependencies").map(|deps| {
            Package::construct_dependencies(deps)
        }).unwrap_or(Vec::new());

        Ok(Package {
            name: name,
            version: version,
            authors: authors,
            dependencies: deps,
            root: root.to_owned(),
            filetree: FileTree::from_root(root).unwrap(),
        })
    }

    fn construct_dependencies(_value: &toml::Value) -> Vec<(String, String)> {
        Vec::new()
    }
}
