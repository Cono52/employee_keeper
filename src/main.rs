use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
extern crate ctrlc;

const DATA_FILE_PATH: &str = "./emp_data.txt";

fn get_or_create_data_file() -> Result<HashMap<String, Vec<String>>, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(DATA_FILE_PATH)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let deserialized: HashMap<String, Vec<String>> = match serde_json::from_str(&contents) {
        Ok(val) => val,
        Err(err) => {
            if err.is_eof() {
                println!("File Empty - Init new map...");
                HashMap::new()
            } else {
                Err(err)?
            }
        }
    };
    Ok(deserialized)
}

fn write_to_data_file(hash_map_data: &HashMap<String, Vec<String>>) -> Result<(), io::Error> {
    let serialized = serde_json::to_string(hash_map_data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(DATA_FILE_PATH)
        .expect("File not found!!!");
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn add_person_to_department(
    company: &mut HashMap<String, Vec<String>>,
    person: &str,
    department: &str,
) -> Result<(), io::Error> {
    match company.get(department) {
        Some(_) => {
            if let Some(dep) = company.get_mut(department) {
                dep.push(person.to_string());
                dep.sort();
            }
        }
        None => {
            company.insert(department.to_string(), vec![person.to_string()]);
        }
    }
    write_to_data_file(&company).expect("update failed!!!");
    Ok(())
}

fn remove_person_from_department(
    company: &mut HashMap<String, Vec<String>>,
    person: &str,
    department: &str,
) -> Result<(), io::Error> {
    match company.get(department) {
        Some(_) => {
            if let Some(dep) = company.get_mut(department) {
                let index = dep
                    .iter()
                    .position(|e| e == person)
                    .expect("Employee doesn't exist in department!");
                dep.remove(index);
                dep.sort();
            }
        }
        None => println!("department doesnt exists..."),
    }
    write_to_data_file(&company).expect("update failed!!!");
    Ok(())
}

fn main() {
    let mut company: HashMap<String, Vec<String>> =
        get_or_create_data_file().expect("Error deserializing data!!!");
    let mut ctrlc_set = false;
    loop {
        if !ctrlc_set {
            ctrlc::set_handler(move || {}).expect("Error setting Ctrl-C handler");
            ctrlc_set = true;
        }

        println!("Please enter command: <op> <name> to <department> - to exit type \"exit\" ");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => n,
            Err(_) => break,
        };

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.len() == 0 {
            continue;
        };

        let operation = tokens[0];

        if operation.to_ascii_lowercase() == "exit" {
            break;
        }

        if operation.to_ascii_lowercase() == "print" {
            for (key, value) in &company {
                println!("Department: {}\n\tEmployees: {}", key, value.join(", "));
            }
            continue;
        }
        if operation.to_ascii_lowercase() == "get" {
            if tokens.len() < 2 {
                println!("department not specified");
                continue;
            }
            let department = tokens[1];
            let value: &Vec<String> = match company.get(department) {
                Some(dep) => dep,
                None => {
                    println!("That department doesnt exist!");
                    continue;
                }
            };
            println!("Employees in {}: {}", department, value.join(", "));
            continue;
        }
        if operation.to_ascii_lowercase() == "add" {
            if tokens.len() < 4 {
                println!("add command has incorrect format!!");
                continue;
            }
            add_person_to_department(&mut company, tokens[1], tokens[3])
                .expect("Something went wrong adding new employee!!");
        }
        if operation.to_ascii_lowercase() == "remove" {
            if tokens.len() < 4 {
                println!("remove command has incorrect format!!");
                continue;
            }
            remove_person_from_department(&mut company, tokens[1], tokens[3])
                .expect("Something went wrong removing employee!!");
        }
    }
}
