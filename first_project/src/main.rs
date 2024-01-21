struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn add_task(description: &str, todo_list: &mut Vec<Task>) {
    let id = todo_list.len() as u32 + 1;
    let new_task = Task {
        id,
        description: String::from(description),
        completed: false,
    };

    todo_list.push(new_task);

}

fn complete_task(id: u32, todo_list: &mut Vec<Task>) -> Option<&Task> {
    for task in todo_list.iter_mut() {
        if task.id == id {
            task.completed = true;
            return Some(task);
        }
    }
    None
}

fn list_tasks(todo_list: &Vec<Task>) {
    println!("ToDo List:");
    for task in todo_list {
        println!(
            "ID: {}, Description: {}, Completed: {}",
            task.id, task.description, task.completed
        );
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();

    let task1 = add_task("Complete coding assignment", &mut todo_list);
    let task2 = add_task("Read a book", &mut todo_list);
    let task3 = add_task("Go for a run", &mut todo_list);

    list_tasks(&todo_list);

    if let Some(completed_task) = complete_task(2, &mut todo_list) {
        println!(
            "\nTask Completed: ID {}, Description: {}, Completed: {}",
            completed_task.id, completed_task.description, completed_task.completed
        );
    } else {
        println!("\nTask with ID 2 not found.");
    }

    list_tasks(&todo_list);
}
