// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me();
    let res = add(1, 2);
    println!("res:{}", res);
}

fn call_me() {}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
