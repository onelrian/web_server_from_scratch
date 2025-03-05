// alias type for short reference
pub type Job = Box<dyn FnOnce() + Send + 'static>;