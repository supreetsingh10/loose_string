pub mod notifier {
    // Works to notify the end user about changes in the file system. 
    use std::path::PathBuf; 
    use crate::common::common::Changes; 

    pub fn show_updates(update: Vec<(PathBuf,Changes)>) {
        if update.len() > 0 {
            for (p, c) in update {
                println!("{:?} {}", p, c); 
            }
        }
    }
}
