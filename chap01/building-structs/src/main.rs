#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL
}

#[derive(Debug)]
struct Human{
    name: String,
    age: i8,
    current_thought: String,
    friend: Friend
}

impl Human {
    fn new(name: String, age: i8,) -> Human {
        Human {
            name: name.to_string(),
            age,
            current_thought: "".to_string(),
            friend: Friend::NIL,
        }
    }
    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        self
    }
    fn with_name(mut self, name: String) -> Human {
        self.name = name;
        self
    }
    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = thought.to_string();
        self
    }
}

fn main() {
    let developer_friend = Human::new("Caroline Morton".to_string(), 30);
    let mut developer = Human::new(String::from("Nguyen Tam"), 20);
    developer = developer.with_friend(Box::new(developer_friend)).with_thought("I love Rust!");
    println!("{:?}", developer);
    println!("{}", developer.name);

}
