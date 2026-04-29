use crate::enums::TaskStatus;
use crate::structs::base::Base;

pub struct Done{
    pub super_struct: Base,
}
impl Done{
    pub fn new(input_title: String)-> Self{
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        Done{super_struct: base}
    }
}