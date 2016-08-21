pub fn raindrops(number: i32) -> String {
  let mut phrase = String::new();
  let pling = "Pling";
  let plang = "Plang";
  let plong = "Plong";

  if number % 3 == 0 {
    phrase.push_str(pling);
  }

  if number % 5 == 0 {
    phrase.push_str(plang);
  }
  
  if number % 7 == 0 {
    phrase.push_str(plong);
  }

  if phrase.is_empty() {
    phrase.push_str(&number.to_string()); 
  }

  phrase
}
