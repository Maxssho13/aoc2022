use nom::character::complete::u64 as parse_u64;
use std::cell::RefCell;
use std::rc::Rc;

const FS_SIZE: u64 = 70_000_000;
const UPDATE_SIZE: u64 = 30_000_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/07.txt")?)?;

    let commands = parse_commands(&input);

    let home_dir = Rc::new(RefCell::new(Dir::new("/")));
    let mut current_dir = Rc::clone(&home_dir);

    let mut all_dirs = Vec::new();

    for command in commands {
        match command {
            ParsedCommand::Cd { path } => {
                if path == ".." {
                    let parent = current_dir.borrow().parent.clone();
                    current_dir = parent.unwrap();
                } else if path == "/" {
                    current_dir = home_dir.clone();
                } else {
                    let selected_dir = {
                        let children_dirs = &current_dir.borrow().children_dirs;
                        let selected_dir = children_dirs
                            .iter()
                            .find(|dir| dir.borrow().name == path)
                            .unwrap();

                        Rc::clone(selected_dir)
                    };

                    current_dir = selected_dir;
                }
            }
            ParsedCommand::Ls { result } => {
                for parsed in result {
                    match parsed {
                        ParsedFile::File(file) => {
                            current_dir.borrow_mut().children_files.push(file);
                        }
                        ParsedFile::Dir { name } => {
                            let mut new_dir = Dir::new(name);
                            new_dir.parent = Some(Rc::clone(&current_dir));

                            let new_dir = Rc::new(RefCell::new(new_dir));
                            all_dirs.push(new_dir.clone());

                            current_dir.borrow_mut().children_dirs.push(new_dir);
                        }
                    }
                }
            }
        }
    }

    let available_space = FS_SIZE - home_dir.borrow_mut().size();
    let space_needed_to_delete = UPDATE_SIZE - available_space;

    let all_dirs_sizes: Vec<u64> = all_dirs.iter().map(|dir| dir.borrow_mut().size()).collect();

    let mut best_size = u64::MAX;
    for size in all_dirs_sizes {
        if size >= space_needed_to_delete && size < best_size {
            best_size = size;
        }
    }

    println!("result: {best_size}");

    Ok(())
}

struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    children_dirs: Vec<Rc<RefCell<Dir>>>,
    children_files: Vec<File>,
    cached_size: Option<u64>,
}

impl Dir {
    fn new(name: impl Into<String>) -> Self {
        Dir {
            name: name.into(),
            parent: None,
            children_dirs: Vec::new(),
            children_files: Vec::new(),
            cached_size: None,
        }
    }

    fn size(&mut self) -> u64 {
        if let Some(size) = self.cached_size {
            return size;
        }

        let mut total_size = 0;
        for file in &self.children_files {
            total_size += file.size;
        }
        for dir in &self.children_dirs {
            total_size += dir.borrow_mut().size();
        }

        self.cached_size = Some(total_size);
        total_size
    }
}

#[derive(Debug)]
struct File {
    size: u64,
}

#[derive(Debug)]
enum ParsedFile {
    File(File),
    Dir { name: String },
}

#[derive(Debug)]
enum ParsedCommand {
    Cd { path: String },
    Ls { result: Vec<ParsedFile> },
}

fn parse_commands(input: &str) -> Vec<ParsedCommand> {
    let mut commands = Vec::new();

    for line in input.lines() {
        if line.starts_with('$') {
            if line.contains("cd") {
                let path = line.split_once("cd ").unwrap().1.to_string();

                commands.push(ParsedCommand::Cd { path })
            } else {
                commands.push(ParsedCommand::Ls { result: Vec::new() })
            }
        } else {
            // we are parsing the result of an 'ls' command
            let most_recent_command = commands.last_mut().unwrap();

            if line.starts_with("dir") {
                let dir_name = line.split_once("dir ").unwrap().1.to_string();
                let dir = ParsedFile::Dir { name: dir_name };
                match most_recent_command {
                    ParsedCommand::Ls { result } => result.push(dir),
                    ParsedCommand::Cd { .. } => panic!("Most recent command cannot be a cd"),
                }
            } else {
                let (_, size) = parse_u64::<&str, ()>(line).unwrap();

                match most_recent_command {
                    ParsedCommand::Ls { result } => result.push(ParsedFile::File(File { size })),
                    ParsedCommand::Cd { .. } => panic!("Most recent command cannot be a cd"),
                }
            }
        }
    }

    commands
}
