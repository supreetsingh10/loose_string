pub mod file_monitor {
    use std::collections::HashMap; 
                                                                    // dir, map<file, sha>
    pub fn initialize_file_monitor(v_sdir: Vec<String>) -> HashMap<PathBuf, HashMap<PathBuf, String>> {
        // let uppercase: Vec<String> = v_sdir.into_iter().map(|d| d.to_string().to_uppercase()).collect(); 
        // println!("{:?}", uppercase); 
        let hash_map: HashMap<PathBuf, HashMap<PathBuf, String>> = get_hash_dir(v_sdir); 
        return hash_map; 
    }

    #[warn(dead_code)]
    fn print_map(hash_map: &mut HashMap<PathBuf, HashMap<PathBuf, String>>) {
        for (folder, file_map) in hash_map {
            println!("{:?}", folder); 
            for(file, sha_val) in file_map {
                println!("Inner path {:?}, {}", file, sha_val); 
            }
        }
    }

    // HashMap Directories -> Hashmap of Files 
    use std::path::{Path, PathBuf}; 
    use sha256::try_digest; 
    use std::fs; 

    fn get_hash_dir(v_sdir:Vec<String>) -> HashMap<PathBuf, HashMap<PathBuf, String>> {
        let mut hash_map = HashMap::new(); 

        for dir in v_sdir {
            walk_dir(PathBuf::from(dir), &mut hash_map); 
        }
        return hash_map; 
    }

    // Add the sha256 of all the files recursively, if it is a folder walk it recursively 
    fn walk_dir(dir: PathBuf, hash_map: &mut HashMap<PathBuf, HashMap<PathBuf, String>>) {
        for entry in fs::read_dir(&dir.as_path()).unwrap() {
            let entry = entry.unwrap(); 

            if entry.file_type().unwrap().is_file() {
                let sha_file = get_hash_file(entry.path().as_path().clone()); 
                let mut file_hash_map: HashMap<PathBuf, String> = HashMap::new(); 
                file_hash_map.insert(entry.path(), sha_file); 
                hash_map.insert(dir.clone(), file_hash_map); 
            } else if entry.file_type().unwrap().is_dir() {
                walk_dir(entry.path(), hash_map); 
            }
        }
    }

    fn get_hash_file(file_path: &Path) -> String {
        let sh2_value = match try_digest(file_path) {
            Ok(val) => val,
            Err(e) => panic!("Failed to get the file, because of this {}", e)
        };

        return sh2_value; 
    }

    fn check_for_changes() {

    }
}
