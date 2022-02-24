pub enum ErrorType {
    CommandLineArgs,
    UnreadableFile
}

pub fn error(problem: ErrorType) {
    match problem {
        ErrorType::CommandLineArgs => println!("Usage: 'hawk filename.hawk"),
        ErrorType::UnreadableFile => println!("Could not read file")
    }
}