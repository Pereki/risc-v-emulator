const VERBOSE: bool = true;

pub fn log(msg: &str) {
    if VERBOSE {
        println!("{}", msg);
    }
}
