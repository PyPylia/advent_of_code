pub mod util;

use std::{cell::RefCell, rc::Rc};

use crate::util::*;

static INPUT: &str = include_str!("input.txt");

fn count_sizes(dir: Rc<RefCell<Directory>>) -> usize {
    let dir = dir.borrow();
    let size = dir.get_size();
    let mut total_size = if size < 100_000 { size } else { 0 };

    for subdir in dir.get_subdirectories() {
        total_size += count_sizes(subdir);
    }

    total_size
}

fn compile_directory_sizes(vector: &mut Vec<usize>, dir: Rc<RefCell<Directory>>) {
    let dir = dir.borrow();
    vector.push(dir.get_size());

    for subdir in dir.get_subdirectories() {
        compile_directory_sizes(vector, subdir)
    }
}

fn change_directory(
    directory: &str,
    current_dir: Rc<RefCell<Directory>>,
) -> Rc<RefCell<Directory>> {
    if directory == ".." {
        current_dir.borrow_mut().get_parent().unwrap()
    } else {
        current_dir
            .borrow_mut()
            .get_child(directory)
            .unwrap()
            .as_directory()
    }
}

fn main() -> anyhow::Result<()> {
    let root = Directory::new_root();
    let mut current_dir = root.clone();

    for line in INPUT.lines() {
        let line: Vec<&str> = line.trim().split(" ").collect();
        match (
            line[0],
            line[1],
            if line.len() >= 3 { line[2] } else { " " },
        ) {
            ("$", "cd", "/") => (),
            ("$", "ls", _) => (),
            ("$", "cd", directory) => current_dir = change_directory(directory, current_dir),
            ("dir", name, _) => current_dir.borrow_mut().add_directory(name),
            (size, name, _) => current_dir.borrow_mut().add_file(name, size.parse()?),
        }
    }

    println!(
        "\nTotal less than 100,000: {}",
        count_sizes(root.clone())
    );

    let needed_space = 30_000_000 - (70_000_000 - root.borrow().get_size());

    let mut sizes = vec![];
    compile_directory_sizes(&mut sizes, root.clone());
    sizes.sort();

    for size in sizes {
        if size >= needed_space {
            println!("Size of deleted directory: {}", size);
            break;
        }
    }

    Ok(())
}
