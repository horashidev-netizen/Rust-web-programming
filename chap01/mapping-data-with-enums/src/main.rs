enum SomeValue {
    StringValue(String),
    IntValue(i32),
}
fn main() {
    let multi_array :[SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("three")),
        SomeValue::IntValue(4),
    ];

    for value in multi_array.iter() {
        match value {
            SomeValue::StringValue(s) => println!("The string is: {}", s),
            SomeValue::IntValue(i) => println!("The number is: {}", i),
        }
    }
}
