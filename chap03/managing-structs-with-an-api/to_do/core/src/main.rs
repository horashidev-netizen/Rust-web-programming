mod enums;
mod apis;
mod structs;
use clap::Parser;
use crate::enums::TaskStatus;
use crate::apis::basic_actions::create::create;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() {
    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status).unwrap();
    let to_do_item = create(
        &args.title,
        status_enum
    );
    println!("{}", to_do_item);
}

