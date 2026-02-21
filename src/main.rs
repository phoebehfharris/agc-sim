mod memory;
mod register_types;

struct RegisterFile {
    accumulator: i16,
    // NOTE: Everything below is actually 15-bits
    lower_product: i16,
}

fn main() {
    println!("Hello, world!");
}
