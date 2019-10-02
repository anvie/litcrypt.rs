extern crate xor;

fn main() {
    let source = &[95, 80, 96, 71, 120, 25, 44, 92, 120, 71, 96, 79, 54];
    let result = xor::xor(source, &[23, 53, 12, 43]);
    if let Ok(string) = std::str::from_utf8(&result) {
        println!("{}", string);
    }
}
