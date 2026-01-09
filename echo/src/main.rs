use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg.contains("target/") {
            continue;
        }
        let value = arg;
        print!("{} ", value);
   }
}
