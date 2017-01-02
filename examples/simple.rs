// Compile this example code and check the compiled output with:
// $ strings ./target/debug/examples/simple | grep Voldemort
// the encrypted one will not print anything (just blank).

#![feature(plugin, custom_attribute)]
#![plugin(litcrypt)]

extern crate xor;

fn main(){
    // uncomment this for plain (non pre-compile-encrypted string)
    // println!("his name is: {}", "Voldemort");
    println!("his name is: {}", lc!("Voldemort"));
}
