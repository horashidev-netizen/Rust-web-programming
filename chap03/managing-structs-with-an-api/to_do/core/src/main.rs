mod enums;
mod apis;
mod structs;

use crate::structs::done::Done;
use crate::enums::TaskStatus;
use crate::structs::pending::Pending;
use crate::apis::basic_actions::create::create;

fn main() {
    let done = TaskStatus::DONE;
    println!("{}", done);
    let done = Done::new("Shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);
    let pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
    let to_do_item = create("washing", TaskStatus::PENDING);
    println!("{}", to_do_item);
}
