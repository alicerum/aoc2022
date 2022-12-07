use std::error::Error;
use std::io::BufRead;
use std::rc::Weak;
use std::{cell::RefCell, io::BufReader, rc::Rc};

const ALL_SIZE: i32 = 70000000;
const REQUIRED: i32 = 30000000;

struct File {
    name: String,
    size: i32,
}

struct Dir {
    name: String,
    parent: Option<Weak<RefCell<Dir>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn root() -> Dir {
        Dir {
            name: "/".to_string(),
            parent: None,
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn with_name_and_parent(name: &str, parent: &Rc<RefCell<Dir>>) -> Dir {
        Dir {
            name: name.to_string(),
            parent: Some(Rc::downgrade(parent)),
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn collect_sizes(&self, sizes: &mut Vec<i32>) -> i32 {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }

        for d in &self.dirs {
            size += d.as_ref().borrow().collect_sizes(sizes);
        }

        sizes.push(size);

        size
    }
}

enum Cmd {
    Cd(String),
    Ls,
}

impl Cmd {
    fn from_str(s: &str) -> Option<Cmd> {
        if s.starts_with("$ cd ") {
            let mut parts = s.split(' ');
            parts.next();
            parts.next();
            Some(Cmd::Cd(parts.next().unwrap().to_string()))
        } else if s.starts_with("$ ls") {
            Some(Cmd::Ls)
        } else {
            None
        }
    }
}

pub fn run(input: BufReader<std::fs::File>) -> std::result::Result<String, Box<dyn Error>> {
    let root_dir = Rc::new(RefCell::new(Dir::root()));
    let mut cur_dir = root_dir.clone();

    let mut ls_mode = false;

    for line in input.lines() {
        let l = line?;
        let l = l.trim();

        //command mode
        if l.starts_with("$ ") {
            ls_mode = false;
            let cmd = match Cmd::from_str(&l) {
                None => {
                    eprintln!("wrong command: {}", l);
                    continue;
                }
                Some(c) => c,
            };

            match cmd {
                Cmd::Cd(dir) => {
                    if dir == "/" {
                        cur_dir = root_dir.clone();
                    } else if dir == ".." {
                        let mut tmp: Option<Rc<RefCell<Dir>>> = None;
                        if let Some(d) = &cur_dir.as_ref().borrow().parent {
                            tmp = d.upgrade();
                        }
                        cur_dir = tmp.unwrap().clone();
                    } else {
                        let mut tmp: Option<Rc<RefCell<Dir>>> = None;
                        {
                            let dirs = &cur_dir.as_ref().borrow().dirs;
                            for d in dirs {
                                if d.as_ref().borrow().name == dir {
                                    tmp = Some(d.clone());
                                    break;
                                }
                            }
                        }
                        if let Some(d) = tmp {
                            cur_dir = d;
                        } else {
                            eprintln!("could not find dir {}", dir);
                        }
                    }
                }
                Cmd::Ls => ls_mode = true,
            }
        } else {
            // file reading mode
            if !ls_mode {
                eprintln!("expected command here");
                continue;
            }

            if l.starts_with("dir ") {
                let mut it = l.split(' ');
                it.next();
                let name = it.next().unwrap();
                cur_dir
                    .as_ref()
                    .borrow_mut()
                    .dirs
                    .push(Rc::new(RefCell::new(Dir::with_name_and_parent(
                        name, &cur_dir,
                    ))));
            } else {
                let mut it = l.split(' ');
                let size = it.next().unwrap();
                let name = it.next().unwrap().to_string();

                let size = size.parse::<i32>().unwrap();

                cur_dir
                    .as_ref()
                    .borrow_mut()
                    .files
                    .push(File { name, size });
            }
        }
    }

    let mut sizes = Vec::new();
    let size = root_dir.as_ref().borrow().collect_sizes(&mut sizes);

    let required_size = REQUIRED - (ALL_SIZE - size);
    sizes.sort();
    for s in sizes {
        if s >= required_size {
            return Ok(format!("delete dir with size {}", s));
        }
    }

    Ok("could not find enough".to_string())
}
