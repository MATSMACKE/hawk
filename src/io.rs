pub enum ErrorType {
    CommandLineArgs,
    UnreadableFile
}

pub fn error(problem: ErrorType, line: usize, relevant_code: &str) {
    match problem {
        ErrorType::CommandLineArgs => println!("Usage: 'hawk filename.hawk"),
        ErrorType::UnreadableFile => println!("Could not read file")
    }
}