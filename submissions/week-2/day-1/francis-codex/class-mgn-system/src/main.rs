#[derive(Clone, Debug, PartialEq)]

pub struct StudentStatus {
    Active, 
    Inactive,
}

pub struct Student {
    pub name: String,
    pub age: u8,
    pub status: StudentStatus,
}

pub struct ClassManager {
    pub students: Vec<Student>,
}

impl ClassManager {
    pub fn new() -> Self {
        ClassManager {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, name: String, age: u8) {
        let student = Student {
            name,
            age,
            status: StudentStatus::Active,
        };
        self.students.push(student);
    }

    pub fn edit_student(&mut self, name: &str, new_name: Option<String>, new_age: Option<u8>) {
        for student in &mut self.students {
            if student.name == name {
                if let Some(n) = new_name {
                    student.name = n;
                }
                if let Some(a) = new_age {
                    student.age = a;
                }
            }
        }
    }

    pub fn update_student_status(&mut self, name: &str, status: StudentStatus) {
        for student in &mut self.students {
            if student.name == name {
                student.status = status;
            }
        }
    }

    pub fn delete_student(&mut self, name: &str) {
        self.students.retain(|student| student.name != name);
    }

    pub  fn list_students(&self) {
        for student in &self.students {
            println!("Name: {}, Age: {}, Status: {:?}", student.name, student.age, student.status);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_register_student
// }
