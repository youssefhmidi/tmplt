pub mod task {
    use std::{
        fmt::{Debug, Display},
        sync::{Arc, Mutex},
    };

    use crate::{
        core::ExecutableCommand,
        logformat,
        logger::writer::{
            LogStatus::{self, *},
            LogWriter,
        },
    };
    // a Task struct has the information about the task
    // and it is giving to a TasksExecutor which execute tasks
    // by their ordered depending if its defered or not
    // the order of what to execute first is determined by the TaskSchedular struct

    pub type OpArcMutex<T> = Option<Arc<Mutex<T>>>;

    #[derive(Clone)]
    pub struct Task {
        _task_fn: Arc<dyn Fn() -> Option<TaskError> + Sync + Send>,
        id: usize,

        pub defered: bool,
    }
    #[derive(Clone)]
    pub struct TaskError {
        id: usize,
    }

    impl Debug for TaskError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Task {} failed", self.id)
        }
    }

    impl Debug for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "( defered : {}, task_id : {} )", self.defered, self.id)
        }
    }

    impl Task {
        pub fn new<O, T>(
            execute: T,
            defered: bool,
            id: usize,
            mutex_logger: OpArcMutex<LogWriter>,
        ) -> Self
        where
            O: Display,
            T: ExecutableCommand<O> + Clone + 'static + Sync + Send,
        {
            let execute_copy = execute.clone();
            let task = move || match execute_copy._execute() {
                Some(out) => {
                    let status: String = LogStatus::Info.into();
                    println!("{}", logformat!(out, status));
                    if let Some(logger) = mutex_logger.clone() {
                        let mut log = logger.lock().unwrap();

                        log.write(format!("{out}"), Info)
                    }
                    None
                }
                None => Some(TaskError { id }),
            };

            Task {
                _task_fn: Arc::new(task),
                defered,
                id,
            }
        }

        pub fn execute_task(&self) -> Result<(), TaskError> {
            match (self._task_fn)() {
                Some(err) => Err(err),
                None => Ok(()),
            }
        }
    }
}
