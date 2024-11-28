# Complete Rust Tutorial - From Start to Finish 🦀

## Part 1: Getting Started 🚀

### 1.1 Installation
```bash
# For Windows: Download rustup-init.exe from https://rustup.rs
# For Linux/MacOS:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.2 Your First Program
```rust
// Save this as main.rs
fn main() {
    println!("Hello! Welcome to Rust!");
}

// Run it:
// rustc main.rs
// ./main
```

## Part 2: Basic Building Blocks 🧱

### 2.1 Variables
```rust
fn main() {
    // Immutable by default
    let name = "John";
    
    // Mutable variables
    let mut age = 25;
    age = 26;    // This works!
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    
    // Printing variables
    println!("Name: {}, Age: {}", name, age);
}
```

### 2.2 Data Types
```rust
fn main() {
    // Numbers
    let number: i32 = 42;         // Integer
    let float_num: f64 = 3.14;    // Float
    
    // Boolean
    let is_active: bool = true;
    
    // Character
    let letter: char = 'A';
    
    // String types
    let text: &str = "Hello";              // String slice
    let string = String::from("Hello");    // String object
    
    // Arrays (fixed length)
    let numbers = [1, 2, 3, 4, 5];
    
    // Vectors (growable arrays)
    let mut vec = vec![1, 2, 3];
    vec.push(4);    // Add element
    
    // Tuples
    let person = ("John", 25);
    println!("Name: {}, Age: {}", person.0, person.1);
}
```

## Part 3: Functions and Control Flow 🔄

### 3.1 Functions
```rust
// Basic function
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value
fn add(a: i32, b: i32) -> i32 {
    a + b    // No semicolon means return this value
}

// Function with multiple returns using tuple
fn get_numbers() -> (i32, i32) {
    (42, 7)
}

fn main() {
    greet("Alice");
    let sum = add(5, 3);
    let (x, y) = get_numbers();
    println!("Sum: {}, Numbers: {}, {}", sum, x, y);
}
```

### 3.2 Control Flow
```rust
fn main() {
    // If-else
    let number = 7;
    if number < 5 {
        println!("Less than 5");
    } else {
        println!("5 or greater");
    }
    
    // Match (like switch but better)
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Something else"),
    }
    
    // Loops
    
    // 1. loop (infinite until break)
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 { break; }
    }
    
    // 2. while loop
    while count > 0 {
        println!("Count: {}", count);
        count -= 1;
    }
    
    // 3. for loop
    for i in 0..5 {    // Range: 0 to 4
        println!("Number: {}", i);
    }
}
```

## Part 4: Ownership - Rust's Special Feature 🎯

### 4.1 Basic Ownership Rules
```rust
fn main() {
    // 1. Each value has an owner
    let s1 = String::from("hello");
    
    // 2. Value moves to new owner
    let s2 = s1;
    // println!("{}", s1);  // Error! s1 is moved
    
    // 3. Clone to keep both
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);  // Works!
}
```

### 4.2 References and Borrowing
```rust
fn main() {
    let mut text = String::from("hello");
    
    // Immutable borrow
    let len = calculate_length(&text);
    println!("Length of '{}' is {}", text, len);
    
    // Mutable borrow
    change_text(&mut text);
    println!("Changed text: {}", text);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_text(s: &mut String) {
    s.push_str(" world");
}
```

## Part 5: Structs and Enums 📦

### 5.1 Structs
```rust
// Define a struct
struct Person {
    name: String,
    age: u32,
}

// Add methods to struct
impl Person {
    // Constructor
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    // Method
    fn introduce(&self) {
        println!("Hi, I'm {} and I'm {}", self.name, self.age);
    }
}

fn main() {
    // Create instance
    let person = Person::new(String::from("John"), 25);
    person.introduce();
}
```

### 5.2 Enums
```rust
// Define an enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Text: {}", text),
        }
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Move { x: 10, y: 20 };
    
    m1.call();
    m2.call();
}
```

## Part 6: Error Handling 🚫

### 6.1 Result and Option Types
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn find_user(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("John"))
    } else {
        None
    }
}

fn main() {
    // Using Result
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
    
    // Using Option
    match find_user(1) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }
    
    // Using unwrap_or
    let user = find_user(2).unwrap_or(String::from("Unknown"));
    println!("User: {}", user);
}
```

## Part 7: Collections 📚

### 7.1 Common Collections
```rust
fn main() {
    // Vector - growable array
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);
    
    // HashMap - key-value pairs
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    
    // String - growable text
    let mut message = String::from("Hello");
    message.push_str(" World!");
}
```

## Part 8: Simple Project - Todo List 📝

```rust
use std::collections::HashMap;

struct TodoList {
    tasks: HashMap<u32, String>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
    
    fn add_task(&mut self, task: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }
    
    fn remove_task(&mut self, id: u32) -> Option<String> {
        self.tasks.remove(&id)
    }
    
    fn list_tasks(&self) {
        for (id, task) in &self.tasks {
            println!("{}: {}", id, task);
        }
    }
}

fn main() {
    let mut todo = TodoList::new();
    
    // Add tasks
    todo.add_task(String::from("Learn Rust"));
    todo.add_task(String::from("Build a project"));
    
    // List tasks
    println!("Current tasks:");
    todo.list_tasks();
    
    // Remove a task
    if let Some(task) = todo.remove_task(1) {
        println!("Removed task: {}", task);
    }
    
    // List remaining tasks
    println!("\nRemaining tasks:");
    todo.list_tasks();
}
```

## Tips for Learning 📝

1. Type the code yourself - don't copy-paste
2. Experiment with changing values
3. Use `rustc --explain E0123` to understand errors
4. Use `cargo doc --open` to read documentation
5. Practice with small projects

## Practice Exercises 💪

1. Modify the Todo list to:
   - Add due dates
   - Mark tasks as complete
   - Save tasks to a file

2. Create a simple calculator that:
   - Takes two numbers and an operator
   - Handles errors properly
   - Supports basic operations (+, -, *, /)

Remember:
- The compiler is your friend
- Take your time with each concept
- Practice regularly
- Don't be afraid to make mistakes!

Need help? Try:
- `rustc --explain` for error explanations
- Official Rust documentation
- Rust Playground (play.rust-lang.org)
