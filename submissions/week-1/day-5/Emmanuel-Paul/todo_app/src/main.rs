use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
        }
    }
}

struct TodoManager {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoManager {
    fn new() -> Self {
        TodoManager {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create a new todo item
    fn create_todo(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        let todo = Todo::new(id, title.clone(), description.clone());
        self.todos.insert(id, todo);
        self.next_id += 1;

        println!(" Created todo with ID {}: '{}'", id, title);
        println!("   Description: {}", description);
        println!("   Status: Not completed\n");

        id
    }

    /// Update a todo item (title and/or description)
    fn update_todo(
        &mut self,
        id: u32,
        new_title: Option<String>,
        new_description: Option<String>,
    ) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            let old_title = todo.title.clone();
            let old_description = todo.description.clone();

            if let Some(title) = new_title {
                todo.title = title;
            }
            if let Some(description) = new_description {
                todo.description = description;
            }

            println!(" Updated todo with ID {}:", id);
            println!("   Title: '{}' -> '{}'", old_title, todo.title);
            println!(
                "   Description: '{}' -> '{}'",
                old_description, todo.description
            );
            println!(
                "   Status: {}\n",
                if todo.completed {
                    "Completed"
                } else {
                    "Not completed"
                }
            );

            true
        } else {
            println!(" Todo with ID {} not found for update\n", id);
            false
        }
    }

    /// Delete a todo item
    fn delete_todo(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.remove(&id) {
            println!("  Deleted todo with ID {}: '{}'", id, todo.title);
            println!("   Description: {}", todo.description);
            println!(
                "   Status: {}\n",
                if todo.completed {
                    "Completed"
                } else {
                    "Not completed"
                }
            );
            true
        } else {
            println!(" Todo with ID {} not found for deletion\n", id);
            false
        }
    }

    /// Edit a todo item (alias for update_todo for clarity)
    fn edit_todo(&mut self, id: u32, new_title: String, new_description: String) -> bool {
        println!("  Editing todo with ID {}...", id);
        self.update_todo(id, Some(new_title), Some(new_description))
    }

    /// Mark a todo as completed
    fn mark_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            let was_completed = todo.completed;
            todo.completed = true;

            println!(
                " Marked todo with ID {} as completed: '{}'",
                id, todo.title
            );
            println!("   Description: {}", todo.description);
            println!(
                "   Status: {} -> Completed\n",
                if was_completed {
                    "Already completed"
                } else {
                    "Not completed"
                }
            );

            true
        } else {
            println!(" Todo with ID {} not found for completion\n", id);
            false
        }
    }

    /// Display all todos
    fn display_all_todos(&self) {
        println!(" All Todos:");
        println!("=============");

        if self.todos.is_empty() {
            println!("No todos found.\n");
            return;
        }

        for todo in self.todos.values() {
    println!(
        "ID: {} | Title: '{}' | Status: {}",
        todo.id,
        todo.title,
        if todo.completed {
            " Completed"
        } else {
            " Not completed"
        }
    );
    println!("Description: {}", todo.description);
    println!("---");
}
    }
}

fn main() {
    println!(" Todo List Application in Rust");
    println!("==================================\n");

    let mut todo_manager = TodoManager::new();

    // Create some todos
    println!(" Creating todos...");
    let todo1_id = todo_manager.create_todo(
        "Learn Rust".to_string(),
        "Study Rust programming language fundamentals".to_string(),
    );

    let todo2_id = todo_manager.create_todo(
        "Build a web app".to_string(),
        "Create a full-stack web application using Rust".to_string(),
    );

    let todo3_id = todo_manager.create_todo(
        "Write documentation".to_string(),
        "Document the todo application code".to_string(),
    );

    // Display all todos
    todo_manager.display_all_todos();

    // Update a todo
    println!(" Updating todos...");
    todo_manager.update_todo(
        todo1_id,
        Some("Master Rust".to_string()),
        Some("Deep dive into advanced Rust concepts and patterns".to_string()),
    );

    // Edit a todo (using the edit function)
    println!("  Editing todos...");
    todo_manager.edit_todo(
        todo2_id,
        "Build a REST API".to_string(),
        "Create a RESTful API using Rust and Actix-web".to_string(),
    );

    // Mark todos as completed
    println!(" Marking todos as completed...");
    todo_manager.mark_completed(todo1_id);
    todo_manager.mark_completed(todo3_id);

    // Display all todos after updates
    todo_manager.display_all_todos();

    // Delete a todo
    println!("  Deleting todos...");
    todo_manager.delete_todo(todo2_id);

    // Try to delete a non-existent todo
    todo_manager.delete_todo(999);

    // Try to update a non-existent todo
    println!(" Attempting to update non-existent todo...");
    todo_manager.update_todo(999, Some("This won't work".to_string()), None);

    // Try to mark a non-existent todo as completed
    println!(" Attempting to mark non-existent todo as completed...");
    todo_manager.mark_completed(999);

    // Final display
    println!(" Final state of todos:");
    todo_manager.display_all_todos();

    println!(" Todo List Application Demo Complete!");
}
