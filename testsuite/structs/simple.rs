struct User {
    name: String,
    email: String,
    password: String,
}

pub fn entry() {
    let mut user: User = create_user();

    user.name = String::from("Overwritten name");
    write_mail(&mut user);
    print_passwd(&user);
}

fn write_mail(user: &mut User) {
    user.email = String::from("Overwritten mail");
}

fn print_passwd(user: &User) {
    println!("passwd: {}", user.password);
    println!("strings: {}", "a" == "a");
}

fn create_user() -> User {
    User {
        name: String::from("Default name"),
        email: String::from("test@mail.com"),
        password: String::from("default"),
    }
}
