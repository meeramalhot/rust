//Each value in Rust has an owner.
//There can only be one owner at a time.
//When the owner goes out of scope, the value will be dropped.


//Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. 
fn main() {
  let s = String::from("hello");

  change(&s);
}

// a reference so data it borrows cannot be mutable
fn change(some_string: &String) {
  some_string.push_str(", world");
}


//SLICING
// let s = String::from("hello");

// equivalent
// let slice = &s[0..2];
// let slice = &s[..2];

fn slice {
  let s = String::from("hello");

  let len = s.len();

  let slice = &s[3..len];
  let slice = &s[3..];
}