// alias type for short reference
type Job = Box<dyn FnOnce() + Send + 'static>;