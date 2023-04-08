use libra::file_monitor::file_monitor as fm; 


fn main() {
    let mut dirs: Vec<String> = std::env::args().skip(1).collect(); 
    if dirs.is_empty() {
        dirs.push(String::from(".")); 
    }

    let hash_map = fm::initialize_file_monitor(dirs); 

    // loop that will go on forever, checking for changes in sha256
}
