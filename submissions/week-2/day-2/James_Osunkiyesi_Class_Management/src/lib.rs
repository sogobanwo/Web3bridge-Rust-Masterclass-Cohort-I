// A class management system that has the name of the student, grade, enum that tracks if student is active or not.
//
// Have the following functions:
// - Function to register student
// - Edit
// - Update
// - Delete functions
// - View function

use std::collections::HashMap;

enum StudentStatus {
    ACTIVE,
    EXPELLED,
    SUSPENDED,
    INACTIVE,
}

struct Student {
    name: String,
    status: StudentStatus,
    grades: HashMap<String, u8>,
}

impl Student {
    fn register(name: String) -> Student {
        Student {
            name,
            status: StudentStatus::ACTIVE,
            grades: HashMap::new(),
        }
    }

    fn register_course(&mut self, subject: &str) {
        if self.grades.contains_key(subject) {
            panic!("Course already registered");
        }
        self.grades.insert(subject.to_string(), 0);
    }

    fn add_score(&mut self, subject: &str, score: u8) {
        if self.grades.contains_key(subject) {
            self.grades
                .entry(subject.to_string())
                .and_modify(|e| *e = score);
        } else {
            panic!("Course not registered");
        }
    }

    fn remove_course(&mut self, subject: &str) {
        if self.grades.contains_key(subject) {
            self.grades.remove(subject);
        } else {
            panic!("Course not registered");
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::PartialEq;
    use super::*;

    #[test]
    fn test_register_student() {
        let student_one = Student::register(String::from("James Osunkiyesi"));

        assert!(&student_one.name.eq(("James Osunkiyesi")));
    }

    #[test]
    fn test_register_course() {
        let mut student_one = Student::register(String::from("James Osunkiyesi"));

        student_one.register_course("Math");

        assert!(student_one.grades.contains_key("Math"));
    }

    #[test]
    fn test_add_score() {
        let mut student_one = Student::register(String::from("James Osunkiyesi"));
        student_one.register_course("Math");

        student_one.add_score("Math", 60);
        assert_eq!(*student_one.grades.get("Math").unwrap(), 60);
    }

    #[test]
    fn test_remove_course() {
        let mut student_one = Student::register(String::from("James Osunkiyesi"));
        student_one.register_course("Math");

        student_one.remove_course("Math");
        assert!(!student_one.grades.contains_key("Math"));
    }
}