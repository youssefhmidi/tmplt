pub mod task_schedular {
    use crate::tasks::Task;

    #[derive(Clone, Debug)]
    pub struct TaskSchedular {
        tasks: Vec<Task>,
        defered_tasks_buf: Vec<Task>,
        pub current_itereations: usize,

        pub current_task: Task,
    }

    impl TaskSchedular {
        pub fn new(mut tasks: Vec<Task>) -> Self {
            let defered_buf: Vec<Task> = TaskSchedular::seperate_defered_tasks(&mut tasks);
            let tasks: Vec<Task> = tasks.iter().filter(|v| !v.defered).cloned().collect();

            if tasks.is_empty() {
                let first_task = defered_buf[0].clone();
                return TaskSchedular {
                    tasks: tasks.to_owned(),
                    defered_tasks_buf: defered_buf,
                    current_itereations: 0,
                    current_task: first_task,
                };
            }

            let first_task = tasks[0].clone();
            TaskSchedular {
                tasks: tasks.to_owned(),
                defered_tasks_buf: defered_buf,
                current_itereations: 0,
                current_task: first_task,
            }
        }

        fn seperate_defered_tasks(tasks: &mut [Task]) -> Vec<Task> {
            let mut buffer: Vec<Task> = vec![];

            for task in tasks.to_owned().clone().iter() {
                if task.defered {
                    buffer.push(task.clone());
                }
            }

            buffer
        }

        pub fn len(&self) -> usize {
            self.tasks.len() + self.defered_tasks_buf.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl Iterator for TaskSchedular {
        type Item = Task;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_itereations == 0 {
                self.current_itereations += 1;
                return Some(self.current_task.clone());
            }

            if self.current_itereations == (self.tasks.len() + self.defered_tasks_buf.len()) {
                return None;
            }

            if self.current_itereations >= self.tasks.len() {
                let current_idx = self.current_itereations - self.tasks.len();
                self.current_task = self.defered_tasks_buf[current_idx].clone();
                self.current_itereations += 1;
                return Some(self.current_task.clone());
            }

            self.current_task = self.tasks[self.current_itereations].clone();
            self.current_itereations += 1;
            Some(self.current_task.clone())
        }
    }
}
