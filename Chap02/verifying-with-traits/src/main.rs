struct AdminUser {
    username: String,
    password: String
}
struct User {
    username: String,
    password: String
}

trait CanEdit {
    fn edit(&self) {
        println!("Admin is editing here");
    }
}
trait CanCreate {
    fn create(&self) {
        println!("Admin is creating here");
    }
}
trait CanDelete {
    fn delete(&self) {
        println!("Admin is deleting here");
    }
}

trait GetUsername {
    fn get_username(&self) -> &str;
}
trait SetUsername {
    fn set_username(&mut self, username: String);
}

impl GetUsername for User {
    fn get_username(&self) -> &str {
        &self.username
    }
}
impl SetUsername for User {
    fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

impl CanCreate for AdminUser {}
impl CanDelete for AdminUser {}
impl CanEdit for AdminUser {}

impl CanEdit for User {
    fn edit(&self) {
        println!("User is editting here!");
    }
}

fn create<T: CanCreate>(user: &T) -> () {
    user.create();
}
fn edit<T: CanEdit>(user: &T) -> () {
    user.edit();
}

fn delete<T: CanDelete>(user: &T) -> () {
    user.delete();
}

fn main() {
    let admin = AdminUser{
        username: "admin".to_string(),
        password: "password".to_string()
    };
    let user = User{
        username: "user".to_string(),
        password: "password".to_string()
    };
    create(&admin);
    edit(&admin);
    edit(&user);
    delete(&admin);
}
