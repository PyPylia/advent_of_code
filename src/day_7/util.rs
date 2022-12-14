use std::{
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    rc::{Rc, Weak},
};

pub struct Directory {
    name: String,
    children: Vec<Item>,
    parent: Weak<RefCell<Directory>>,
    this: Weak<RefCell<Directory>>,
}

pub struct File {
    name: String,
    size: usize,
    parent: Rc<RefCell<Directory>>,
}

impl Directory {
    pub fn new_root() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|this| {
            RefCell::new(Directory {
                name: "/".to_string(),
                children: vec![],
                parent: Weak::new(),
                this: this.to_owned(),
            })
        })
    }

    pub fn add_directory(&mut self, name: &str) {
        self.children.retain(|x| x.get_name() != name);

        self.children.push(Item::Directory(Rc::new_cyclic(
            |this| {
                RefCell::new(Directory {
                    name: name.to_string(),
                    children: vec![],
                    parent: self.this.clone(),
                    this: this.to_owned(),
                })
            },
        )))
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        self.children.retain(|x| x.get_name() != name);

        self.children.push(Item::File(Rc::new(RefCell::new(
            File {
                name: name.to_string(),
                size: size,
                parent: self.this.upgrade().unwrap(),
            },
        ))))
    }

    pub fn get_child(&mut self, name: &str) -> Option<Item> {
        for item in &self.children {
            if item.get_name() == name {
                return Some(item.clone());
            }
        }

        None
    }

    pub fn get_size(&self) -> usize {
        let mut size: usize = 0;

        for item in &self.children {
            size += item.get_size()
        }

        size
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Directory>>> {
        self.parent.upgrade()
    }

    pub fn get_subdirectories(&self) -> Vec<Rc<RefCell<Directory>>> {
        let mut subdirs = vec![];

        for child in &self.children {
            if let Item::Directory(directory) = child {
                subdirs.push(directory.clone())
            }
        }

        subdirs
    }
}

impl File {
    pub fn get_parent(&self) -> Rc<RefCell<Directory>> {
        self.parent.clone()
    }
}

#[derive(Clone)]
pub enum Item {
    Directory(Rc<RefCell<Directory>>),
    File(Rc<RefCell<File>>),
}

impl Item {
    pub fn get_name(&self) -> String {
        match self {
            Item::Directory(directory) => directory.as_ref().borrow().name.clone(),
            Item::File(file) => file.as_ref().borrow().name.clone(),
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Item::Directory(directory) => directory.as_ref().borrow().get_size(),
            Item::File(file) => file.as_ref().borrow().size,
        }
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Directory>>> {
        match self {
            Item::Directory(directory) => directory.as_ref().borrow().get_parent(),
            Item::File(file) => Some(file.as_ref().borrow().get_parent()),
        }
    }

    pub fn as_directory(self) -> Rc<RefCell<Directory>> {
        match self {
            Item::Directory(directory) => directory,
            Item::File(_file) => panic!("Expected directory"),
        }
    }

    pub fn as_file(self) -> Rc<RefCell<File>> {
        match self {
            Item::Directory(_directory) => panic!("Expected file"),
            Item::File(file) => file,
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "- {} (file, size={})",
            self.name, self.size
        )
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Directory(directory) => write!(f, "{:?}", directory.as_ref().borrow()),
            Item::File(file) => write!(f, "{:?}", file.as_ref().borrow()),
        }
    }
}

impl Debug for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "- {} (dir, size={}) {:#?}",
            self.name,
            self.get_size(),
            self.children
        )
    }
}
