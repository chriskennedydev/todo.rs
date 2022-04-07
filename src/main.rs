use std::env::{args,var};
use std::fs::{create_dir,File};
use std::io::{BufRead,BufReader,ErrorKind,Write};
use std::path::{Path, PathBuf};

use std::env::consts;

fn main() {
    let argv: Vec<String> = args().collect();
    if argv.len() == 1 {
        println!("Need at least one argument!");
        return;
    }

    let mut home = var("HOME");
    if consts::OS == "windows" {
        home = var("USERPROFILE");
    }

    let todo_dir = Path::new(&home.unwrap()).join(".todo");
    let todo_file = Path::new(&todo_dir).join("todo");

    if !Path::new(&todo_dir).exists() {
        let _r = create_todo_dir(todo_dir);
    }

    if !Path::new(&todo_file).exists() {
        let _ = File::create(todo_file.clone());
    }

    let fd = todo_file;
    let mut todo_num: u8 = 1;
    let cmd = &argv[1];

    if cmd == "del" || cmd == "update" || cmd == "done" {
        todo_num = argv[2].parse().unwrap();
    }

    let _cmd = match cmd.as_str() {
        "add" => {
            let todos = argv[2..].to_vec();
            add_todo(todos, fd)
        }
        "update" => {
            let todos = argv[3..].to_vec();
            update_todo(todo_num, todos, fd)
        }
        "del" => delete_todo(todo_num, fd),
        "done" => complete_todo(todo_num, fd),
        "list" => list_todo(fd),
        _ => usage(),
    };
}

fn add_todo(todo: Vec<String>, file: PathBuf) -> std::io::Result<()> {
    let mut fd = File::options().read(true).append(true).open(file)?;
    let mut long_todo = String::new();
    if todo.len() > 1 {
        for word in todo {
            long_todo.push_str(&word);
            long_todo.push_str(" ");
        }
    } else {
        long_todo.push_str(&todo[0]);
        long_todo.push_str(" ");
    }
    long_todo.push_str("\n");
    let r = fd.write_all(long_todo.as_bytes());
    let _r = match r {
        Ok(r) => r,
        Err(error) => panic!("Cannot write to file {:?} {}", fd, error),
    };
    Ok(())
}

fn delete_todo(todo_num: u8, file: PathBuf) -> std::io::Result<()> {
    let fp = file.clone();
    let f = load_file(file);

    let mut todos = Vec::new();
    let reader = BufReader::new(f);
    for (_, todo) in reader.lines().enumerate() {
        todos.push(todo);
    }

    let mut fd = File::create(fp).unwrap();
    for (ind, todo) in todos.iter().enumerate() {
        if ind + 1 != todo_num.into() {
            let _r = fd.write_all(todo.as_ref().unwrap().as_bytes());
            let r = fd.write_all("\n".as_bytes());
            let _r = match r {
                Ok(r) => r,
                Err(error) => println!("Cannot write to file: {:?}", error),
            };
        }
    }
    Ok(())
}

fn complete_todo(todo_num: u8, file: PathBuf) -> std::io::Result<()> {
    let fp = file.clone();
    let f = load_file(file);

    let mut todos = Vec::new();
    let reader = BufReader::new(f);

    for (_, todo) in reader.lines().enumerate() {
        todos.push(todo);
    }

    let mut fd = File::create(fp).unwrap();

    for (ind, todo) in todos.iter().enumerate() {
        if ind + 1 == todo_num.into() {
            let _ = fd.write_all(todo.as_ref().unwrap().as_bytes());
            let _ = fd.write_all("✓".as_bytes());
            let r = fd.write_all("\n".as_bytes());
            let _r = match r {
                Ok(r) => r,
                Err(error) => println!("Cannot write to file: {:?}", error),
            };
        } else {
            let _ = fd.write_all(todo.as_ref().unwrap().as_bytes());
            let r = fd.write_all("\n".as_bytes());
            let _r = match r {
                Ok(r) => r,
                Err(error) => println!("Cannot write to file: {:?}", error),
            };
        }
    }
    Ok(())
}

fn update_todo(todo_num: u8, updated_todo: Vec<String>, file: PathBuf) -> std::io::Result<()> {
    let fp = file.clone();
    let f = load_file(file);

    let mut long_todo = String::new();

    if updated_todo.len() > 1 {
        for word in updated_todo {
            long_todo.push_str(&word);
            long_todo.push_str(" ");
        }
    } else {
        long_todo.push_str(&updated_todo[0]);
        long_todo.push_str(" ");
    }
    long_todo.push_str("\n");

    let mut todos = Vec::new();
    let reader = BufReader::new(f);

    for (_, todo) in reader.lines().enumerate() {
        todos.push(todo);
    }

    let mut fd = File::create(fp).unwrap();

    for (ind, todo) in todos.iter().enumerate() {
        if ind + 1 == todo_num.into() {
            let r = fd.write_all(long_todo.as_bytes());
            let _r = match r {
                Ok(r) => r,
                Err(error) => println!("Cannot write to file: {:?}", error),
            };
        } else {
            let _ = fd.write_all(todo.as_ref().unwrap().as_bytes());
            let r = fd.write_all("\n".as_bytes());
            let _r = match r {
                Ok(r) => r,
                Err(error) => println!("Cannot write to file: {:?}", error),
            };
        }
    }
    Ok(())
}

fn list_todo(file: PathBuf) -> std::io::Result<()> {
    let f = load_file(file);

    let reader = BufReader::new(f);

    for (ind, todo) in reader.lines().enumerate() {
        println!("{}. {}", ind + 1, todo.unwrap());
    }
    Ok(())
}

fn create_todo_dir(todo_dir: PathBuf) -> Result<(), &'static str> {
    let result = create_dir(todo_dir);
    let _ = match result {
        Ok(r) => r,
        Err(error) => panic!("Unable to create directory: {}", error),
    };
    Ok(())
}

fn load_file(file: PathBuf) -> File {
    let f = File::open(file.as_path()).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file.clone()).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    f
}

fn usage() -> std::io::Result<()> {
    println!("Usage!");
    Ok(())
}
