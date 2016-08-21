pub fn hello(name: Option<&str>) -> String {
  if let Some(n) = name {
    format!("Hello, {}!", n)
  } else {
    format!("Hello, World!")
  }
}
