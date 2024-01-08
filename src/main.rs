use rand::Rng;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

struct TodoList {
    list: Vec<Task>,
}

impl TodoList {
    fn add_task(&mut self, description: &str) -> Task {
        let new_task = Task {
            id: rand::thread_rng().gen(),
            description: String::from(description),
            completed: false,
        };
        let new_task_copy = new_task.clone();
        self.list.push(new_task);
        new_task_copy
    }
    fn complete_task(&mut self, id: u32) -> Option<&Task> {
        match self.list.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.completed = true;
                Some(task)
            }
            None => None,
        }
    }

    fn list_tasks(&self) {
        for task in &self.list {
            println!(
                "Id: {} | Description: {} | Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList { list: Vec::new() };

    let task1 = todo_list.add_task("Buy groceries");
    let _task2 = todo_list.add_task("Read a book");

    todo_list.complete_task(task1.id);

    todo_list.list_tasks();
}
