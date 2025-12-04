fn main() {
    let current_path = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_path);
    let tsp_instance = tsp_parser::parse_tsp_instance("instances/bench/first.tsp");
    println!("{:?}", tsp_instance);
}
