// use std::env;
use std::io::{self, Write};
// use std::fmt;
// use std::fs;

mod todo_list;
use todo_list::ToDoList;

fn main() {
    // For opening saved todo-lists:
    // let args: Vec<String> = env::args().collect();
    // let todo_list_path = &args[1];

    let mut todo_list = ToDoList::new();

    loop {
        if todo_list.get_len() > 0 {
            todo_list.print();
        }

        println!("\n[OPTIONS]:");
        println!("1. Add a new task.");
        println!("2. Remove a task.");
        println!("3. Switch task completion.");
        println!("4. Swap tasks.");
        println!("5. Exit");

        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let choice: u8 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("Specify the task at hand: ");
                io::stdout().flush();
                let mut description = String::new();
                io::stdin().read_line(&mut description);
                todo_list.add_task(description.trim().to_string());
                println!("Task has been ADDED.");
            }
            2 => {
                print!("Enter the index of a task to remove: ");
                io::stdout().flush();
                let mut index = String::new();
                io::stdin().read_line(&mut index);
                let index: usize = index.trim().parse().unwrap_or(0) - 1;
                todo_list.remove_task(index);
                println!("Task has been REMOVED.");
            }
            3 => {
                print!("Enter the index of a task in which you want to switch completion: ");
                io::stdout().flush();
                let mut index = String::new();
                io::stdin().read_line(&mut index);
                let index: usize = index.trim().parse().unwrap_or(0) - 1;
                todo_list.switch_taskCompletion(index);
            }
            4 => {

            }
            5 => {
                break;
            }
            _ => println!("Bad argument. Try again."),
        }
    }
}
