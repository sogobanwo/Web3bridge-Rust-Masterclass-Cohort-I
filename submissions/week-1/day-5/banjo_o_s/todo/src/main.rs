fn main() {
    let mut todo_list: VecTodo = VecTodo::new();
    let new_id: u32 = todo_list.id_counter;

    let todo: Todo = Todo::new(new_id, "Learn Rust".to_string(), "read the rust book".to_string());
    todo_list.add_todo(todo);
    println!("Todo List: {:?}", todo_list);

    let new_id: u32 = todo_list.id_counter;
    let another_todo: Todo = Todo::new(new_id, "gym".to_string(), "".to_string());
    todo_list.add_todo(another_todo);

    println!("{:#?}", todo_list);

    // mark to completed
    // first get the todo from the list
    let found_todo: Option<(usize, &mut Todo)> = todo_list.get_todo(new_id);

    let (_, todo) = match found_todo {
        Some((i, v)) => (i, v),
        None => {
            println!("could not locate todo");
            return;
        },
    };
    todo.mark_completed();


    println!("after completed::: {:#?}", todo_list);

}

#[derive(Debug)]
struct VecTodo {
    todos: Vec<Todo>,
    id_counter: u32,
}


#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl VecTodo {

    fn new() -> Self {
        Self {
            todos: Vec::new(),
            id_counter: 1,
        }
    }

    fn add_todo(&mut self, todo: Todo) -> bool {
        // add the todo to the list
        self.todos.push(todo);

        // increment the id counter
        self.id_counter += 1;
        true
    }

    fn delete_todo(&mut self, id: u32) -> bool {
        // self.
        true
    }

    fn update_todo(&mut self, index: usize, todo: &Todo) -> bool {
        true
    }

    fn get_todo(&mut self, id: u32) -> Option<(usize, &mut Todo)> {
        for (index, todo) in self.todos.iter_mut().enumerate() {
            if todo.id == id {
                return Some((index, todo));
            }
        }
        None
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

    fn mark_completed(&mut self) {
        self.completed = true;
        return;
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