pub fn greet_user(name: Option<String>) {
  match name {
    Some(name) => println!("Hello there, {}!", name),
    None => println!("Well howdy, stranger!"),
  }
}
