pub mod common {
    pub enum Changes {
        Creation,
        Deletion,
        Modified
    }

    use std::fmt as fmt; 

    // Required to format the Enum values. 
    impl fmt::Display for Changes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Changes::Creation => write!(f, "Creation"),
                Changes::Deletion => write!(f, "Deletion"),
                Changes::Modified => write!(f, "Modified"),
            }
        }
    }
}
