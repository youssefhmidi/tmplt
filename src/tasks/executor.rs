pub mod task_executor {
    use core::fmt;
    use std::thread::{self, JoinHandle};

    use crate::tasks::{Task, TaskError, TaskSchedular};
    pub type ThreadResult = JoinHandle<Option<TaskError>>;

    #[derive(Debug, Clone)]
    pub struct TasksExecutor {
        _task_schedular: TaskSchedular,
        batch_size: usize,
        /// the `switch` field is a toggle so that the taskExecutor will only give batchs of not defered
        /// task and stops untill it turns on  
        switch: bool,
        current_iterations: usize,
        temp_buf: Vec<Task>,

        pub current_batch: Vec<Task>,
    }
    #[derive(Clone)]
    pub struct ExecutionError;

    impl fmt::Debug for ExecutionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "faild to execute batchs , found empty batch.")
        }
    }

    pub fn execute_batch(batch: Vec<Task>) -> Result<ThreadResult, ExecutionError> {
        if batch.is_empty() {
            return Err(ExecutionError);
        }

        let thread = thread::spawn(move || -> Option<TaskError> {
            for task in batch {
                match task.execute_task() {
                    Err(e) => return Some(e),
                    Ok(_) => continue,
                }
            }
            None
        });

        Ok(thread)
    }
    impl TasksExecutor {
        pub fn new(task_schedular: TaskSchedular, batch_size: usize) -> Self {
            let current_batch: Vec<Task> = Vec::with_capacity(batch_size);

            TasksExecutor {
                _task_schedular: task_schedular,
                current_batch,
                batch_size,
                current_iterations: 0,
                switch: false,
                temp_buf: vec![],
            }
        }

        pub fn toggle_switch(&mut self) -> &mut Self {
            self.switch = !self.switch;

            self
        }
    }

    impl Iterator for &mut TasksExecutor {
        type Item = Vec<Task>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut next_batch: Vec<Task> = self
                ._task_schedular
                .clone()
                .skip(self.current_iterations * self.batch_size)
                .take(self.batch_size)
                .take_while(|v| !v.defered || self.switch)
                .collect();

            if next_batch.is_empty() {
                // we decrease it by one so if the switch is on in the next iteration it will return the
                // defered tasks
                self.current_iterations -= 1;
                return None;
            }

            if self.current_iterations * self.batch_size >= self._task_schedular.len() {
                return None;
            }

            if self.batch_size >= self._task_schedular.len() {
                self.current_iterations += 1;
                let mut next_batch: Vec<Task> = self
                    ._task_schedular
                    .clone()
                    .take_while(|v| !v.defered || self.switch)
                    .collect();
                // a filter if the batch size is bigger than tasks and to filter only defered tasks
                if self.switch {
                    let mut filtered = next_batch
                        .iter()
                        .filter(|v| v.defered)
                        .map(|v| v.to_owned())
                        .collect::<Vec<Task>>();
                    next_batch.clear();
                    next_batch.append(&mut filtered);
                }

                let leftover_batch: Vec<Task> =
                    self._task_schedular.clone().filter(|t| t.defered).collect();

                self.temp_buf = leftover_batch;

                if !self.temp_buf.is_empty() && self.switch {
                    let temp = self.temp_buf.clone();
                    self.temp_buf.clear();
                    return Some(temp);
                }

                if next_batch.is_empty() {
                    return None;
                }

                return Some(next_batch);
            }

            if self.switch {
                let mut filtered = next_batch
                    .iter()
                    .filter(|v| v.defered)
                    .map(|v| v.to_owned())
                    .collect::<Vec<Task>>();
                next_batch.clear();
                next_batch.append(&mut filtered);
            }

            self.current_iterations += 1;
            self.current_batch = next_batch.clone();
            Some(next_batch)
        }
    }
}
