mod generator;
mod wordbank;

fn main() {
    println!("Compiling modules...");
    let _gen = generator::WeldGenerator::new();
    println!("Weld Engine is ready.");
}
