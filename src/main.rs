use std::env;
use std::fs;
use std::io::{Read, Write};
use std::process::Command;
use std::path::Path;

const CONFIG_PATH: &str = "cmd_alias";
const CONFIG_NAME: &str = "aliases.cmd";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let c_args: Vec<String> = env::args().collect();
    let mut action = {
        if c_args.len() < 2 {
            "list"
        } else {
            &c_args[1].as_str()
        }
    };
    let args = {
        if c_args.len() < 2 {
            String::new()
        } else if c_args.len() < 3
            && (
                action == "set" || action == "add" ||
                action == "del" || action == "remove"
            ) {
            action = "list";
            String::new()
        } else {
            c_args[2..c_args.len()].join(" ")
        }
    };

    let config_root = Path::new("C:\\Users\\E\\AppData\\Roaming");

    let config_root = config_root.join(CONFIG_PATH);
    let config_file = config_root.join(CONFIG_NAME);
    let _ = fs::create_dir_all(&config_root).unwrap();

    match action {
        "list" | "ls" => {
            list_alias(&config_file);
        },
        "set" | "add" => {
            set_alias(&config_file, &args);
            load_alias(&config_file);
        },
        "del" | "remove"  => {
            del_alias(&config_file, &args);
            load_alias(&config_file);
        },
        "load" | "source" | "silent" | "s" => {
            load_alias(&config_file);
        },
        "help" => {
            print_help();
        },
        _ => ()
    }

    Ok(())
}

fn print_help() {
    println!(
r#"
Alias (custom-eqsue-implemetation) v0.1.1
Einjerjar 2021

alias [set key=value] [del key] [load]

[None]      List all set aliases
set k=v     Sets an alias
del k       Deletes an alias
load        Load alias silently
"#
    );
}

fn config_contents(config_file: &Path) -> String{
    let mut ss = String::new();
    fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(config_file)
        .unwrap()
        .read_to_string(&mut ss)
        .unwrap();

    ss
}

fn list_alias(config_file: &Path) {
    let conf = config_contents(config_file);
    let items = conf.split('\n');

    for item in items {
        if item.trim().is_empty() | item.starts_with("@echo") {
            continue;
        }
        let item = item.replace("doskey ", "");
        let item = item.split_once('=').unwrap();

        println!("{} = {}", item.0, item.1);
    }
}
fn set_alias(config_file: &Path, args: &str) {
    let conf = config_contents(config_file);
    let items = conf.split('\n');
    let arg_items = args.split_once('=').unwrap();

    let mut conf_h = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(config_file)
        .unwrap();

    let mut found = false;

    let _ = writeln!(&mut conf_h, "@echo off");
    for item in items {
        if item.trim().is_empty() | item.starts_with("@echo") {
            continue;
        }
        let item = item.replace("doskey ", "");
        let item = item.split_once('=').unwrap();

        let mut x = format!("doskey {}={}", item.0, item.1);
        if item.0 == arg_items.0 {
            x = format!("doskey {}={}", arg_items.0, arg_items.1);
            found = true;
        }

        let _ = writeln!(&mut conf_h, "{}", x);
    }

    if !found {
        let _ = writeln!(&mut conf_h, "{}", format!("doskey {}={}", arg_items.0, arg_items.1));
    }

    let _ = writeln!(&mut conf_h, "@echo on");
}
fn del_alias(config_file: &Path, args: &str) {
    let conf = config_contents(config_file);
    let items = conf.split('\n');

    let mut conf_h = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(config_file)
        .unwrap();

    let _ = writeln!(&mut conf_h, "@echo off");
    for item in items {
        if item.trim().is_empty() | item.starts_with("@echo") {
            continue;
        }
        let item = item.replace("doskey ", "");
        let item = item.split_once('=').unwrap();

        if item.0 == args {
            let _ =
                Command::new("cmd")
                    .arg("/C")
                    .arg(format!("doskey {}=", args).as_str())
                    .output();
            continue;
        }

        let _ = writeln!(&mut conf_h, "{}", format!("doskey {}={}", item.0, item.1));
    }

    let _ = writeln!(&mut conf_h, "@echo on");
}

fn load_alias(config_file: &Path) {
    let _ =
        Command::new(config_file.as_os_str())
            .output()
            .expect("[@ALIAS: What");
}



