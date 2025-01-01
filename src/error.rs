#[derive(Debug)]
pub enum Status {
    // An Actor failed to perform an action. Triggers a fallback.
    ActionFail, 
    // An unexpected error. Panic.
    Error(&'static str),
}

pub type Result<T> = std::result::Result<T, Status>;
