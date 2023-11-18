use crate::arg_parser::args::Options;
use crate::core;
use crate::Parser;
use crate::logger::writer::LogWriter;
use crate::tasks::OpArcMutex;
use crate::tasks::TasksExecutor;
use crate::tasks::{execute_batch, ThreadResult, ExecutionError, TaskError};

fn execute(mut outs: Vec<Result<ThreadResult, ExecutionError>>, executor: &mut TasksExecutor) {
    for task_batch in executor {
        let out = execute_batch(task_batch);
        outs.push(out)
    } 

    let outs = outs.into_iter().collect::<Result<Vec<ThreadResult>, ExecutionError>>();

    let thread_results = match outs{
        Ok(res) => res,
        Err(e) => return eprintln!("{e:?}"),
    };

    let mut op_errors: Vec<Option<TaskError>> = vec![];

    for thread in thread_results {
        op_errors.push(thread.join().unwrap());
    }

    let _ = op_errors.iter().map(|op| {
        match op {
            Some(err) => return eprintln!("{err:?}"),
            None => ()
        }
    }).collect::<Vec<_>>();
}

pub fn generate(args: Options, logger: OpArcMutex<LogWriter>) {
    let file_parser = Parser::new("tmplt".to_string());
    let file = match args.get_file() {
        Some(file) => file,
        None => {
            return eprintln!("should've passed a file ( no file dedected that end with .tmplt )")
        }
    };


    let out = file_parser.parse_file(file);

    let mut interpreter = match out {
        Ok(vect) => {
            
            let tree = core::construct_tree(vect).unwrap();

            let mut interpreter =core::construct_interpreter(tree);
            match interpreter.interpret() {
                Ok(interpreter) => interpreter.to_owned(),
                Err(e) => return eprintln!("{:?}", e),
            }
        }
        Err(e) => return eprint!("{}", e),
    };

    let mut executor = match interpreter.create_tasks_executor(args.get_batch_size_flag(), logger.clone()) {
        Some(exe) => exe,
        None => panic!("wtf just happened, paniced while creating tasks (this shouldn't happen)"),
    };

    // first iteration
    execute(vec![], &mut executor);
    
    executor.toggle_switch();
    // second iteration
    execute(vec![], &mut executor);
}
