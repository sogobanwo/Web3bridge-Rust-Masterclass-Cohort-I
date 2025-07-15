struct Student{
    name: String,
    grade, u8,
    status: Status  
}

enum Status{
    active,
    inactive,
}

struct School{
    students: Vec<Student>,
}

impl School{
    fn new() -> School{
        School {
            students: Vec::new(),
        }
    },

    fn add_student(&mut self, name: &str, grade: u8, status: Status) {
        let student = Student {
            name: String::from(name),
            grade,
            status,
        };
        self.students.push(student);
    }

    fn edit_student(&mut self, name: &str, new_name: &str, new_grade: u8, new_status: Status){
        self.students.iter_mut().find(|student| student.name == name ){
            student.grade = new_grade;
            student.status = new_status;
        };
    }

    fn remove_student(&mut self, name:&str){
        self.students.retain(|student| student.name != name);
    }

    fn view_students(&self){
        self.students
    }

}
fn main() {
    println!("Hello, world!");
}
