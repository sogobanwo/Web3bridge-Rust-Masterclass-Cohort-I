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