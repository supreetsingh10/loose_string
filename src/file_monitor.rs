pub mod file_monitor {
    use std::collections::HashMap; 
                                                                    // dir, map<file, sha>
    pub fn initialize_file_monitor(v_sdir: Vec<String>) -> HashMap<PathBuf, Vec<HashMap<PathBuf, String>>> {
        // let uppercase: Vec<String> = v_sdir.into_iter().map(|d| d.to_string().to_uppercase()).collect(); 
        // println!("{:?}", uppercase); 
        let mut hash_map: HashMap<PathBuf, Vec<HashMap<PathBuf, String>>> = get_hash_dir(v_sdir); 
        print_map(&mut hash_map);
        return hash_map; 
    }

    fn print_map(hash_map: &mut HashMap<PathBuf, Vec<HashMap<PathBuf, String>>>) {
        for (folder, file_map) in hash_map {
            println!("{:?}", folder); 
            for in_file in file_map {
                for(inner_key, inner_val) in in_file {
                    println!("Inner key {:?}, Inner Value {:?}", inner_key, inner_val); 
                }
            }
        }
    }

    // HashMap Directories -> Hashmap of Files 
    use std::path::{Path, PathBuf}; 
    use sha256::try_digest; 
    use std::fs; 

    fn get_hash_dir(v_sdir:Vec<String>) -> HashMap<PathBuf, Vec<HashMap<PathBuf, String>>> {
        let mut hash_map = HashMap::new(); 

        for dir in v_sdir {
            walk_dir(PathBuf::from(dir), &mut hash_map); 
        }

        hash_map
    }

    // Add the sha256 of all the files recursively, if it is a folder walk it recursively 
    fn walk_dir(dir: PathBuf, hash_map: &mut HashMap<PathBuf, Vec<HashMap<PathBuf, String>>>) {
        for entry in fs::read_dir(&dir.as_path()).unwrap() {
            let entry = entry.unwrap(); 

            if entry.file_type().unwrap().is_file() {
                let sha_file = get_hash_file(entry.path().as_path().clone()).unwrap(); 
                let mut file_hash_map: HashMap<PathBuf, String> = HashMap::new(); 
                file_hash_map.insert(entry.path(), sha_file); 
                // check if the key exists, else make a new vector then push the key, otherwise,
                // get the value and then push the value in the present vector. 
                // hash_map.insert(dir.clone(), file_hash_map); 
                //
                if hash_map.contains_key(&dir) {
                    // get mutable value; 
                    let value = hash_map.get_mut(&dir).unwrap(); 
                    value.push(file_hash_map); 
                } else {
                    // Add the key-value by making a new vector;
                    let mut path_vec: Vec<HashMap<PathBuf, String>> = Vec::new(); 
                    path_vec.push(file_hash_map); 
                    hash_map.insert(dir.clone(), path_vec); 
                }
            } else if entry.file_type().unwrap().is_dir() {
                walk_dir(entry.path(), hash_map); 
            }
        }
    }

    fn get_hash_file(file_path: &Path) -> Option<String> {
         match try_digest(file_path) {
            Ok(val) => {
                return Some(val)
            },
            Err(_e) => { 
                return None
            },
        };
    }

    pub enum Changes {
        Creation,
        Deletion,
        Modified
    }

    pub fn updates(hash_map: &mut HashMap<PathBuf, Vec<HashMap<PathBuf, String>>>) -> Vec<(PathBuf, Changes)> {
        let changed_files = check_for_modifications(hash_map); 
        let created_files = check_for_creation(hash_map); 
        changed_files
    }

    // Since the path bufs of the directories are present 
    //  dir -> Vec({dir, sha256})
    //  We can check for deletions in this file itself if, the pathbuf does not exist then it means
    //  it has been deleted. 
    fn check_for_modifications(hash_map: &mut HashMap<PathBuf, Vec<HashMap<PathBuf, String>>>) -> Vec<(PathBuf, Changes)> {
        // Returns the Vector of changed files which have PathBufs
        let mut changed_files: Vec<(PathBuf, Changes)> = Vec::new(); 
        for (_dir, file_map) in hash_map {
            for inner_file in file_map {
                for (inner_file_path, inner_sha_value) in inner_file {
                    // Check if the earlier existing paths have been deleted or modified
                    if let Some(changed_sha) = get_hash_file(inner_file_path.as_path()) {
                        // if sha256 has changed
                        if changed_sha.as_str() != inner_sha_value.as_str() {
                            changed_files.push((inner_file_path.clone(), Changes::Modified)); 
                        }
                    } else {
                        // if the existing files were deleted.
                        changed_files.push((inner_file_path.clone(), Changes::Deletion)); 
                    }
                }
            }
        }
        changed_files
    }

    // Check for creation
    //
    fn if_created_file(vec_map: &Vec<HashMap<PathBuf, String>>, file: &PathBuf) -> bool {
        for m in vec_map.iter() {
            if m.contains_key(file.as_path()) {
                return true; 
            }
        }

        return false; 
    }

    fn check_for_creation(hash_map: &mut HashMap<PathBuf, Vec<HashMap<PathBuf, String>>>) -> Vec<(PathBuf, Changes)> {
        let mut created_files: Vec<(PathBuf, Changes)> = Vec::new(); 

        for(dir, file_map) in hash_map.clone() {
            for entry in fs::read_dir(dir.clone().as_path()).unwrap() {
                let entry = entry.unwrap(); 

                if entry.file_type().unwrap().is_dir() {
                    if !hash_map.contains_key(&entry.path()) {
                        created_files.push((entry.path(), Changes::Creation))
                    }
                } else if entry.file_type().unwrap().is_file() {
                    //let created = if_created_file(hash_map.get_mut(dir.as_path()).unwrap(), &entry.path()); 
                    if !if_created_file(&file_map, &entry.path()) {
                        created_files.push((entry.path(), Changes::Creation)); 
                    }
                }
            }

            // check if entry is a file or a folder, 
            // if it is not logged in the vector then it is created. 
            // write a function for checking if it is in the vector or not
        }
        created_files 
    }
}
