/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Command {
    Add { name: String, department: String },
    List { department: String },
    All,
    Exit,
}

impl Command {
    fn from_string(s: &str) -> Option<Self> {
        let cmd: Vec<&str> = s.split_whitespace().collect();

        match cmd.as_slice() {
            ["Add", name, "to", department] => Some(Self::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["List", department] => Some(Self::List {
                department: department.to_string(),
            }),
            ["All"] => Some(Self::All),
            ["Exit"] => Some(Self::Exit),
            _ => None,
        }
    }
}

type Department = String;
type Person = String;

#[derive(Debug)]
struct Departments {
    departments: HashMap<Department, Vec<Person>>,
}

impl Departments {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn process(&mut self, cmd: Command) -> bool {
        match cmd {
            Command::Add { name, department } => {
                self.add_to_department(department, name);
                false
            }
            Command::List { department } => {
                self.print_department(&department);
                false
            }
            Command::All => {
                if self.departments.is_empty() {
                    println!("No Departments found.")
                } else {
                    for department in self.departments.keys() {
                        self.print_department(department)
                    }
                }

                false
            }
            Command::Exit => {
                println!("Exiting!");
                // Return true to signify an exit
                true
            }
        }
    }

    fn add_to_department(&mut self, department: String, name: String) {
        self.departments
            .entry(department)
            .or_default()
            .push(name)
    }

    fn print_department(&self, department: &Department) {
        match self.departments.get(department).cloned() {
            Some(mut people) => {
                people.sort();
                println!("People in {department}: {:?}", people)
            }
            None => println!("Department {department} is empty."),
        }
    }
}

fn main() {
    // Define departments
    let mut departments = Departments::new();

    // Define a string for the users command
    let mut cmd = String::new();

    loop {
        // Empty the existing command
        cmd.clear();

        println!("Please input your command:");

        // Read the command
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        // Parse the command
        match Command::from_string(&cmd) {
            Some(cmd) => {
                if departments.process(cmd) {
                    break;
                }
            }
            None => println!("Invalid Command!"),
        }
    }
}
