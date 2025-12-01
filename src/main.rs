use heap_memory::Box;

fn main() {
    let mut b = Box::new(String::from("Hello, "));
    println!("{}", *b);

    b.push_str("friend");
    println!("{}", *b);
}
