use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::io::{Cursor, BufRead};
use std::path::{PathBuf, Path};
use std::process::{Command, Output};

use ::package::{Package, FileTree};
use ::error::Error;

struct CoqDep<'p> {
    package: &'p Package
}

fn to_logical_path(root: &Path, path: &Path) -> String {
    let mut logical_path = String::new();

    let mut first = true;

    for chunk in path.relative_from(root).unwrap().iter() {
        if !first {
            logical_path.push('.');
            first = false;
        }
        logical_path.push_str(chunk.to_str().unwrap());
    }

    debug!("to_logical_path: mapping physical path {:?} to {}", path, logical_path);

    logical_path
}

impl<'p> CoqDep<'p> {
    fn run(self) -> Dependencies {
        fn recursive_run(pkg: &Package, path: &Path, filetree: &FileTree, deps: &mut Dependencies) {
            for child in filetree.directories.get(path).unwrap() {
                // If the child entry is a directory we want to recursively process it.
                if filetree.directories.contains_key(child) {
                    // First ensure we have encounted for any depdencies here.
                    recursive_run(pkg, child, filetree, deps);
                    // We then process the child pulling out
                    let output = CoqDep::invoke(pkg, child, filetree).output().unwrap();
                    for (products, ds) in CoqDep::parse_output(output.stdout) {
                        // println!("found products {:?} that needs {:?}", products, deps)
                        for product in &products {
                            for dep in &ds {
                                deps.nodes.insert(product.clone());
                                deps.nodes.insert(dep.clone());
                                let edge = deps.edges.remove(dep);
                                let mut updated_edge = edge.unwrap_or(Vec::new());
                                updated_edge.push(product.clone());
                                deps.edges.insert(dep.clone(), updated_edge);
                            }
                        }
                    }
                }
            }

            let output = CoqDep::invoke(pkg, path, filetree).output().unwrap();

            for (products, ds) in CoqDep::parse_output(output.stdout) {
                // println!("found products {:?} that needs {:?}", products, deps)
                for product in &products {
                    for dep in &ds {
                        deps.nodes.insert(product.clone());
                        deps.nodes.insert(dep.clone());
                        let edge = deps.edges.remove(dep);
                        let mut updated_edge = edge.unwrap_or(Vec::new());
                        updated_edge.push(product.clone());
                        deps.edges.insert(dep.clone(), updated_edge);
                    }
                }
            }
        }

        let mut deps = Dependencies::empty();
        recursive_run(&self.package, &self.package.root, &self.package.filetree, &mut deps);
        return deps;
    }

    fn invoke(pkg: &Package, directory: &Path, filetree: &FileTree) -> Command {
        let directory_files = filetree.directories.get(directory).unwrap();

        debug!("Coqc::invoke: files={:?}", directory_files);
        // First we classify the directories which must be mapped via `-R`, and files which
        // are passed directly to CoqDep.
        let mut files = Vec::new();
        let mut directories = Vec::new();

        for file in directory_files {
            if filetree.directories.contains_key(file) {
                directories.push(file)
            } else {
                files.push(file)
            }
        }

        // We then construct the command to be run
        let mut cmd = Command::new("coqdep");

        if directory.to_owned() != pkg.root {
            cmd.arg("-Q")
               .arg(directory)
               .arg("");
        }

        for dir in directories {
            let logical_path = to_logical_path(&pkg.root, dir);

            cmd.arg("-R")
               .arg(dir.to_str().unwrap())
               .arg(logical_path);
        }

        for file in files {
            cmd.arg(file);
        }

        debug!("invoke: command={:?}", cmd);

        return cmd;
    }

    fn parse_output(output: Vec<u8>) -> Vec<(Vec<PathBuf>, Vec<PathBuf>)> {
        let output = Cursor::new(output);
        output.lines().map(|line| {
            let line = line.unwrap();
            debug!("parse_output: PARSED_LINE={:?}", line);
            let split_line: Vec<_> = line.split(':').collect();

            // Each line should be of the form product : depdencies.
            assert!(split_line.len() == 2);

            let products = split_line[0].split(" ")
                                        .filter(|s| s != &"" )
                                        .map(|x| PathBuf::from(x.to_owned()))
                                        .filter(|p| p.extension().unwrap() == "v" ||
                                                    p.extension().unwrap() == "vo")
                                        .map(|p| p.with_extension("v"))
                                        .collect::<Vec<_>>();

            let deps = split_line[1].split(" ")
                                    .filter(|s| s != &"" )
                                    .map(|x| PathBuf::from(x.to_owned()))
                                    .filter(|p| p.extension().unwrap() == "v" ||
                                                p.extension().unwrap() == "vo")
                                    .map(|p| p.with_extension("v"))
                                    .collect::<Vec<_>>();

            let result = (products, deps);
            debug!("parse_output: result={:?}", result);
            result
        }).filter_map(|pair| match pair {
            (prods, deps) => if prods.len() == 0 { None } else { Some((prods, deps))}
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct Dependencies {
    nodes: HashSet<PathBuf>,
    edges: HashMap<PathBuf, Vec<PathBuf>>
}

impl Dependencies {
    fn empty() -> Dependencies {
        Dependencies {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn is_dependent(&self, product: &Path, dep: &Path) -> bool {
        debug!("is_dependent: product={:?} dep={:?}", product, dep);
        // This is sort of hack, should handle this when building depdencies.
        if product == dep {
            return false;
        }

        match self.edges.get(dep) {
            None => false,
            Some(products) => {
                products.iter()
                        .filter(|p| dep.to_owned() != *p.to_owned())
                        .any(|p| product.to_owned() == p.to_owned() || self.is_dependent(product, p))
            }
        }
    }
}

/// A programmatic interface to the Coq compiler.
#[derive(Debug)]
pub struct Coqc<'p> {
    pub package: &'p Package,
}

fn execute_command(mut command: Command) -> Result<Output, Error> {
    debug!("execute_command: command={:?}", command);
    let output = try!(command.output());
    if output.status.success() {
        Ok(output)
    } else {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("sub-command failed with: {}\n\n{}", stdout, stderr);
    }
}

impl<'p> Coqc<'p> {
    pub fn run(self) -> Result<(), Error> {
        fn recursive_run(pkg: &Package,
                         path: &Path,
                         deps: &mut Dependencies,
                         directories: &mut Vec<PathBuf>,
                         files: &mut Vec<PathBuf>) -> Result<(), Error> {
            for child in pkg.filetree.directories.get(path).unwrap() {
                if pkg.filetree.directories.contains_key(child) {
                    // First ensure we have encounted for any depdencies here.
                    let mut files = Vec::new();
                    try!(recursive_run(pkg, child, deps, directories, &mut files));
                    directories.push(child.to_owned());
                } else {
                    files.push(child.to_owned());
                }
            }

            sort_dependencies(files, deps);

            debug!("sorted files: {:?}", files);

            let compiler = Coqc::invoke(pkg, path, directories, files);

            try!(execute_command(compiler));

            Ok(())
        }

        let coqdep = CoqDep { package: &self.package };
        let mut dependecy_graph = coqdep.run();
        let mut files = Vec::new();
        let mut directories = Vec::new();

        try!(recursive_run(&self.package,
                           &self.package.root,
                           &mut dependecy_graph,
                           &mut directories,
                           &mut files));

        Ok(())
    }

    fn invoke(pkg: &Package, directory: &Path, directories: &Vec<PathBuf>, files: &Vec<PathBuf>) -> Command {
        let mut cmd = Command::new("coqc");

        if directory.to_owned() != pkg.root {
            cmd.arg("-R")
               .arg(directory)
               .arg(to_logical_path(&pkg.root, directory));
        }

        for dir in directories {
            let logical_path = to_logical_path(&pkg.root, dir);

            cmd.arg("-R")
               .arg(dir.to_str().unwrap())
               .arg(logical_path);
        }

        for file in files {
            cmd.arg(file);
        }

        cmd
    }
}

fn sort_dependencies(files: &mut Vec<PathBuf>, deps: &Dependencies) {
    debug!("sort_dependencies: at_start={:?}", files);
    files.sort_by(|x, y| {
        if deps.is_dependent(x, y) {
            Ordering::Greater
        } else if deps.is_dependent(y, x) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    debug!("sort_dependencies: at_end={:?}", files);
}

// impl Compiler for Coqc {
//     type Error = Error;

//     fn run(root: &Path, package: Package) -> Result<(), Error> {
//         panic!()
//     }
// }
