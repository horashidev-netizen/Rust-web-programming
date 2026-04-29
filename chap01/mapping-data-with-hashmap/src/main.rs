use std::collections::HashMap;

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Item(Vec<String>)
}

fn main() {
    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();
    profile.insert("name", CharacterValue::Name("Tam Nguyen Duc".to_string()));
    profile.insert("age", CharacterValue::Age(20));
    profile.insert("item", CharacterValue::Item(vec!["Laptop".to_string(), "Keyboard".to_string()]));

    println!("{:?}", profile);

    match profile.get("name") {
        Some(value_data) => {
            match value_data {
                CharacterValue::Name(s) => println!("The string is: {}", s),
                _ => panic!("name should be a string")
            }
        }
        None => {println!("name is not present");}
    }
}