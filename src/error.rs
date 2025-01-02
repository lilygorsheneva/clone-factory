#[derive(Debug, PartialEq)]
pub enum Status {
    // An Actor failed to perform an action. Triggers a fallback.
    ActionFail, 
    // A world state update failed to apply. This breaks any semblance of atomicity.
    StateUpdateError,
    // An unexpected error. Panic.
    Error(&'static str),
}

pub type Result<T> = std::result::Result<T, Status>;
