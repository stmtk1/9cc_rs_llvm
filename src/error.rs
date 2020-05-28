#[derive(Debug)]
pub enum ErrorType {
    UnexpectedToken,
    UnexpectedChar,
    UnexpectedEOL,
}

#[derive(Debug)]
pub struct QccError {
    pub kind: ErrorType,
    pub pos: usize,
    pub message: String,
}


impl std::fmt::Display for QccError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.pos {
            write!(f, " ")?;
        }
        write!(f, "^\n{}", self.message)
    }
}
