use libra::file_monitor::file_monitor as fm; 
use libra::notifier::notifier as nf; 

fn main() {
    let mut dirs: Vec<String> = std::env::args().skip(1).collect(); 
    if dirs.is_empty() {
        dirs.push(String::from(".")); 
    }

    let mut hash_map = fm::initialize_file_monitor(dirs); 

    // loop that will go on forever, checking for changes in sha256
    // Include the metadata when the file was changed and what was changed. 
    loop {
        let v_updated = fm::updates(&mut hash_map); 
        nf::show_updates(v_updated); 
    }
}
