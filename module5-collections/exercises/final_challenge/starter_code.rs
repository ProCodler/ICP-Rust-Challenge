// Starter code for the Rust Task Manager challenge

// Add serde derives for Task and TaskStatus
#[derive(serde::Serialize, serde::Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }
    
    // List all tasks
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("[{}] {} - {}{}", task.id, task.title, match task.status { TaskStatus::Pending => "Pending", TaskStatus::Completed => "Completed" }, if let Some(ref d) = task.due_date { format!(" (Due: {})", d) } else { String::new() });
        }
    }

    // Mark a task as completed
    fn complete_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            true
        } else {
            false
        }
    }

    // Delete a task by id
    fn delete_task(&mut self, id: u32) -> bool {
        let len_before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < len_before
    }

    // Filter tasks by status
    fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.status as u8 == status as u8).collect()
    }

    // Save tasks to a file (simple JSON)
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.tasks).unwrap();
        std::fs::write(filename, json)
    }

    // Load tasks from a file (simple JSON)
    fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let data = std::fs::read_to_string(filename)?;
        self.tasks = serde_json::from_str(&data).unwrap();
        if let Some(max_id) = self.tasks.iter().map(|t| t.id).max() {
            self.next_id = max_id + 1;
        } else {
            self.next_id = 1;
        }
        Ok(())
    }
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();
    
    // Main application loop
    loop {
        // TODO: Get user command
        
        
        // TODO: Process command
        
        // TODO: Exit if command is Quit
        break; // Temporary break to avoid infinite loop
    }
}