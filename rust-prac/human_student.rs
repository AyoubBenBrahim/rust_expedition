// Define a trait to represent common behavior of humans
trait Human {
    fn new(name: String, age: u32) -> Self;
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

// Implement the Human trait for a generic struct
struct GenericHuman {
    name: String,
    age: u32,
}

impl Human for GenericHuman {
    fn new(name: String, age: u32) -> Self {
        GenericHuman { name, age }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

// Define a struct for a student inheriting from GenericHuman
struct Student {
    human: GenericHuman,
    student_id: u32,
}

impl Student {
    fn new(name: String, age: u32, student_id: u32) -> Self {
        let human = GenericHuman::new(name, age);
        Student {
            human,
            student_id,
        }
    }

    fn get_student_id(&self) -> u32 {
        self.student_id
    }
}

// Implement the Human trait for Student by delegating to GenericHuman's implementation
impl Human for Student {
    fn new(name: String, age: u32) -> Self {
        Student::new(name, age, 0) // Student needs an additional parameter for student_id
    }

    fn get_name(&self) -> &String {
        self.human.get_name()
    }

    fn get_age(&self) -> u32 {
        self.human.get_age()
    }
}

fn main() {
    let human = GenericHuman::new("John".to_string(), 30);
    println!("Name: {}", human.get_name());
    println!("Age: {}", human.get_age());

    let student = Student::new("Alice".to_string(), 20, 1234);
    println!("Name: {}", student.get_name());
    println!("Age: {}", student.get_age());
    println!("Student ID: {}", student.get_student_id());
}

