// a Task struct has the information about the task
// and it is giving to a TasksExecutor which execute tasks
// by their ordered depending if its defered or not
// the order of what to execute first is determined by the TaskSchedular struct
mod executor;
mod schedular;
mod task;

pub use executor::task_executor::*;
pub use schedular::task_schedular::*;
pub use task::task::*;

