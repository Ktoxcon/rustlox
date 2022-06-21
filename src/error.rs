#[derive(Debug)]
pub struct CompilationError {
    pub line: usize,
    pub message: String,
}
