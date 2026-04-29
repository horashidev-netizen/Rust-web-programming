use crate::enums::TaskStatus;
use crate::structs::base::Base;

pub struct Pending {
    pub super_struct: Base,
}
impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        Pending { super_struct: base }
    }
}


#[test]
fn test_pending() {
    let pending = Pending::new("test");
    assert_eq!("test", pending.super_struct.title);
    assert_eq!(pending.super_struct.status, TaskStatus::PENDING);

}