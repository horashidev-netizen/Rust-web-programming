mod api;
mod enums;
mod structs;

use clap::Parser;
use crate::enums::TaskStatus;
use crate::structs::done::Done;
use crate::structs::pending::Pending;

/// Simple program for booking
#[derive(Debug, Parser)]
struct Args {
    ///First name of user
    #[arg(short, long)]
    first_name: String,

    ///last name of user
    #[arg(short, long)]
    last_name: String,

    ///age of the user
    #[arg(short, long, default_value_t = 1)]
    age: u16
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("{:?}", args.first_name);
    println!("{:?}", args.last_name);
    println!("{:?}", args.age);
    println!("{}", TaskStatus::DONE);
    println!("{}", TaskStatus::PENDING);
    let outcome = TaskStatus::DONE.to_string();
    println!("{}", outcome);

    let done = Done::new("Shopping".to_string());
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);
    let pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
}
