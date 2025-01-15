use std::io;

struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

fn get_options() -> String {
    println!("Choose from the options");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to select options");
    input.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task description:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take description");
    input.trim().to_string();
    tasks.push(Task::new(input));
}

fn view_task(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("There is no task");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        let iscompelted = if task.completed {
            "Completed"
        } else {
            "Pending"
        };
        println!("{} , {} , {}", i + 1, iscompelted, task.description)
    }
}

fn edit_task(tasks: &mut [Task]) {
    view_task(tasks);

    println!("Enter the index of task you want to Edit");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to get index");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            return;
        }
    };

    if index > tasks.len() {
        println!("Invalid Index");
        return;
    }

    let task = &tasks[index];
    println!("Editing Task: - {}", task.description);

    println!("Enter new description (leave blank to keep current):");
    let mut new_description = String::new();

    io::stdin()
        .read_line(&mut new_description)
        .expect("Failed to get input");

    let new_description = new_description.trim();

    if !new_description.is_empty() {
        tasks[index].description = new_description.to_string();
    }

    println!("Task updated successfully!");
}

fn mark_as_completed(tasks: &mut [Task]) {
    view_task(tasks);

    println!("Enter the index of task you want to mark as complete");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to get index");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            return;
        }
    };

    if index > tasks.len() {
        println!("Invalid Index");
        return;
    }

    tasks[index-1].completed = true;

    println!("Task Completed")
}

fn remove_task(tasks: &mut [Task]) {
    view_task(tasks);

    println!("Enter then index you want to remove");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to get index");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            return;
        }
    };
    if index > tasks.len() {
        println!("Invalid Index");
        return;
    }

    println!("Task removed successfully");
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nTODO List:");
        println!("1. Add tasks");
        println!("2. View a task");
        println!("3. Remove a task");
        println!("4. Edit Task");
        println!("5. Mark task as completed");
        println!("6. Exit");

        let choice: String = get_options();

        match choice.as_str() {
            "1" => {
                add_task(&mut tasks);
            }
            "2" => {
                view_task(&tasks);
            }
            "3" => {
                remove_task(&mut tasks);
            }
            "4" => {
                edit_task(&mut tasks);
            }
            "5" => {
                mark_as_completed(&mut tasks);
            }
            "6" => {
                println!("Exiting the TODO List. Goodbye!");
                break; 
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
