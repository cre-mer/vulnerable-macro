use dangerous_proc_macro_lib::return_42;

#[return_42]
fn my_function() -> u32 {
    // this function can do whatever, we implement the attribute to always return 42 🤗
}

fn main() {
    println!("My function's result: {}", my_function());
    println!("Macro has been executed!");
}
