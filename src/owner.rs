struct User {
    name: String,
    email: String,
    active: bool,
}
fn main() {
    // let s = "hello world";
    let mut s = String::from("hello world");
    println!("{}", s);
    s.push_str("string");
    let _ss = s;
    println!("{}", _ss);

    let user: User = User { name: String::from("mung"), email: String::from("1234"), active: true };
    println!("{}", user.name);
    println!("{}", user.email);
    println!("{}", user.active);
    let name = user.name;
    println!("{}", name);
    println!("{}", user.name); // user.name의 소유권이 name으로 이동했기 때문에 에러
}
