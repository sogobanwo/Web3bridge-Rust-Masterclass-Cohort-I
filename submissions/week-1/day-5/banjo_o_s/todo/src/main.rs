fn main() {
    let mut todo_list: VecTodo = VecTodo::new();
    
}

struct VecTodo {
    todos: Vec<Todo>,
}


#[derive(Debug)]
struct Todo {
    id: u32,
    tile: String,
    description: String,
    completed: bool,
}

impl VecTodo {

    fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    fn add_todo(&mut self, todo: Todo) -> bool {
        self.todos.push(todo);
        true
    }

    fn delete_todo(&mut self, id: u32) -> bool {
        true
    }

    fn get_todo(&self, id: u32) -> Option<&Todo> {
        Option::None
    }
}


impl Todo {

    fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) -> bool {
        self.completed = true;
        true
    }

    fn update_todo(&mut self, title: String, description: String) -> bool {
        self.title = title;
        self.description = description;
        true
    }

    fn edit_todo(&mut self, description: String) -> bool {
        self.description = description;
        true
    }

    // fn delete_todo()
}