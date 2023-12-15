pub mod interpreter {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::process::Command;
    use std::{fmt, fs};

    use crate::core::Tokens::{SectionIdentity, Token};
    use crate::core::{Branch, Tree};
    use crate::logformat;
    use crate::logger::writer::{LogWriter, LogStatus};
    use crate::tasks::{Task, TaskSchedular, TasksExecutor, OpArcMutex};

    #[derive(Clone, Debug)]
    pub enum BufferType {
        CommandLine(Vec<CommandSerializer>),
        FsAction(Vec<ExacutableFsAction>),
    }

    /// a Interpreter struct contains the syntax tree and the variables
    #[derive(Clone, Debug)]
    pub struct Interpreter {
        /// hash map of format (var_name : var_value)
        _variable_buf: HashMap<String, String>,
        _commands_buf: Vec<CommandSerializer>,
        _fs_actions_buf: Vec<ExacutableFsAction>,

        pub syntax_tree: Tree,
    }

    // todo make a more reliable error message
    #[derive(Clone, Default)]
    pub struct InterpreterError {
        when: String,
        reason: String,
    }

    impl InterpreterError {
        pub fn new(when: &str, reason: &str) -> Self {
            InterpreterError {
                when: when.to_string(),
                reason: reason.to_string(),
            }
        }
    }

    impl fmt::Debug for InterpreterError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Interpreter Error: when {}, {}", self.when, self.reason)
        }
    }

    impl fmt::Display for Interpreter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(
                f,
                " variables buffer : {:?}\n commands buffer : {:?}\n FileSystem actions : {:?}",
                self._variable_buf, self._commands_buf, self._fs_actions_buf
            )
        }
    }

    impl Interpreter {
        pub fn construct(syntax_tree: Tree) -> Self {
            Interpreter {
                _variable_buf: HashMap::new(),
                syntax_tree,
                _commands_buf: vec![],
                _fs_actions_buf: vec![],
            }
        }

        /// interpret the syntax tree and store variables in its own buffer and commands in a seperate buffer
        /// and create tasks after serilizing the command and the variables it uses
        pub fn interpret(&mut self) -> Result<&mut Self, InterpreterError> {
            for branch in self.syntax_tree.branches.clone() {
                if branch.section_kind == SectionIdentity::Variables {
                    self.variable_parsing(branch)?;
                }
            }

            for branch in self.syntax_tree.branches.clone() {
                let mut command_serializers: Vec<CommandSerializer> = vec![];
                let mut fs_actions: Vec<ExacutableFsAction> = vec![];
                match branch.section_kind {
                    SectionIdentity::CurrentWorkingDirectory => {
                        self.dir_structure_parser(branch, &mut fs_actions)?
                    }
                    SectionIdentity::Demostration => {
                        self.demos_parser(branch, &mut command_serializers)?
                    }
                    SectionIdentity::Scripts => {
                        self.scripts_parsing(branch, &mut command_serializers)
                    }
                    _ => continue,
                }
                self._commands_buf.append(&mut command_serializers);
                self._fs_actions_buf.append(&mut fs_actions)
            }
            self._variable_buf.clear();

            Ok(self)
        }

        pub fn create_tasks_executor(
            &mut self,
            batch_size: &usize,
            op_logger: OpArcMutex<LogWriter>,
        ) -> Option<TasksExecutor> {
            let fs_action_op = self._fs_actions_buf.clone();
            let cmd_action = self
                ._commands_buf
                .clone()
                .iter_mut()
                .map(|v| v._serialize_to_cmd())
                .collect::<Vec<ExecutableTerminalCommand>>();

            let mut tasks: Vec<Task> = vec![];
            let mut i: usize = 0;

            for action in fs_action_op {
                let deferd = action.2;
                let thread_logger = match op_logger.clone() {
                    Some(logger) => Some(logger),
                    None => None,
                };

                let task = Task::new(action, deferd, i, thread_logger);
                i += 1;
                tasks.push(task)
            }

            for action in cmd_action {
                let defered = action.1;
                let thread_logger = match op_logger.clone() {
                    Some(logger) => Some(logger),
                    None => None,
                };
                
                let task = Task::new(action, defered, i, thread_logger);
                i += 1;
                tasks.push(task)
            }

            let schedular = TaskSchedular::new(tasks);
            let executor = TasksExecutor::new(schedular, *batch_size);

            Some(executor)
        }

        fn variable_parsing(&mut self, variable_branch: Branch) -> Result<(), InterpreterError> {
            for node in variable_branch.nodes.clone() {
                if node.get_words().len() != 3 {
                    return Err(InterpreterError::new("interpreting variables", "not enough, or more tokens has been used, 
                                                                                            \rhint: consider checking your syntax"));
                }

                if !node.get_words().contains(&"=".to_string()) {
                    return Err(InterpreterError::new("interpreting variables", "didn't find a '=' token, Note that you can only declare vaiables in this section"));
                }
            }

            for node in variable_branch.nodes {
                let variable: Vec<String> =
                    node.filter(|v| v.0 != Token::Assign).map(|v| v.1).collect();

                self._variable_buf
                    .insert(variable[0].clone(), variable[1].clone())
                    .unwrap_or("".to_string());
            }
            Ok(())
        }

        fn scripts_parsing(
            &mut self,
            branch: Branch,
            command_serializers: &mut Vec<CommandSerializer>,
        ) {
            for node in branch.nodes {
                let mut defered = false;
                let command_name = {
                    if let Token::DeferAction = Token::from(node.get_words()[0].clone()) {
                        defered = true;
                        node.get_words()[1].clone()
                    } else {
                        node.get_words()[0].clone()
                    }
                };
                let mut command_args: Vec<String> = vec![];

                for (tk, val) in node {
                    match tk {
                        Token::Arg(mut s) => {
                            s.remove(0);
                            command_args.push(self._variable_buf[&s].clone())
                        }
                        Token::Skip => {
                            if val != command_name {
                                command_args.push(val)
                            }
                        }
                        _ => continue,
                    }
                }

                let new_command = CommandSerializer::new(command_name, command_args, defered);
                command_serializers.push(new_command)
            }
        }

        fn demos_parser(
            &mut self,
            branch: Branch,
            command_serializers: &mut Vec<CommandSerializer>,
        ) -> Result<(), InterpreterError> {
            let err = "interpreting the DEMO sections";

            for (line, node) in branch.nodes.iter().enumerate() {
                if node.get_words().len() > 4 {
                    return Err(InterpreterError::new(
                        err,
                        "too many tokens you can only have 3 to 4 tokens in one line",
                    ));
                }

                let mut defered = false;
                let second_token = match Token::from(node.get_words()[0].clone()) {
                    Token::DeferAction =>{
                        defered = true;
                        Token::from(node.get_words()[2].clone())
                    },
                    Token::Skip =>
                        Token::from(node.get_words()[1].clone()),
                    _ => return Err(InterpreterError::new(err, format!("found invalid token at {} in the demo section, all the valid tokens are COPY_INTO or DEFER", line + 1).as_str()))
                };
                if second_token != Token::CopyAction {
                    return Err(InterpreterError::new(err, 
                    format!("unable to interpret line {} in the demo section, found an unexpected token .", line+1).as_str()));
                }

                let filtered = node
                    .get_words()
                    .iter()
                    .map(|v| {
                        // make it replace every variable
                        v
                    })
                    .filter(|v| v.contains('/'))
                    .map(|v| v.clone())
                    .collect::<Vec<String>>();

                if filtered.len() != 0 {
                    return Err(InterpreterError::new(
                        err,
                        format!("make sure to use backslashes '\\' error occurs at {}", line)
                            .as_str(),
                    ));
                }

                let args: Vec<String> = node
                    .get_words()
                    .iter()
                    .filter(|v| {
                        (Token::from(v.to_string()) != Token::CopyAction)
                            && (Token::from(v.to_string()) != Token::DeferAction)
                    })
                    .map(|v| v.clone())
                    .collect();

                let command = CommandSerializer::new("copy".to_string(), args, defered);

                command_serializers.push(command)
            }
            Ok(())
        }

        fn dir_structure_parser(
            &mut self,
            branch: Branch,
            fs_actions: &mut Vec<ExacutableFsAction>,
        ) -> Result<(), InterpreterError> {
            let err = "interpreting the directories structure";

            for (line, node) in branch.nodes.iter().enumerate() {
                let words = node.get_words();
                if words.len() > 3 {
                    return Err(InterpreterError::new(err, format!("unable to interpret at line {} in the directories section, found too mush tokens.", line).as_str()));
                }

                if words.len() == 3 && words[0].to_uppercase() != "DEFER".to_string() {
                    return Err(InterpreterError::new(err, format!("Unvalid token at {}:0 in the directories section expected DEFER found {}",line + 1 ,words[0].clone()).as_str()));
                }

                let mut defered = false;
                let mut is_file = false;
                let mut path = String::new();

                for (tk, val) in node.clone() {
                    match tk {
                        Token::DeclareFile => is_file = true,
                        Token::DeferAction => defered = true,
                        Token::Arg(mut v) => {
                            v.remove(0);
                            path = self._variable_buf[&v].clone()
                        }
                        Token::Skip => path = val,
                        _ => (),
                    }
                }

                let fs_action = ExacutableFsAction(path, is_file, defered);
                fs_actions.push(fs_action)
            }
            Ok(())
        }
    }

    pub trait ExecutableCommand<O>
    where
        O: Display,
    {
        fn _execute(&self) -> Option<O>;
    }

    /// contains a command that will be serielized to be executable
    #[derive(Clone, Debug)]
    pub struct CommandSerializer {
        _args: Vec<String>,

        pub command_name: String,
        pub defered: bool,
    }

    /// a named tuple for specifiy if the command should be executed without order or should be defered
    ///
    /// simple format (args, defered)
    #[derive(Debug, Clone)]
    pub struct ExecutableTerminalCommand(
        /// a vector of strings (`Vec<String>`) that represent the args to the command that will be run
        Vec<String>,
        /// this `boolean` represent if the current action should be executed in the order it is placed in
        /// or to run after all task has finished
        pub bool,
    );


    /// a named tuple to simplify the creatio of a file/directory
    ///
    /// simple format (path, is_file, defered)
    #[derive(Debug, Clone)]
    pub struct ExacutableFsAction(
        /// this `string` represent the path of the file or the directory to create
        String,
        /// this `boolean` represent if the fs action is to create a file or a directory
        bool,
        /// this `boolean` represent if the current action should be executed in the order it is placed in
        /// or to run after all task has finished
        pub bool,
    );

    #[derive(Clone)]
    pub struct CmdOut {
        stdout: String,
        stderr: String,
    }

    impl CmdOut {
        pub fn new(stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
            let msg = "unable to parse String from utf8, Details: the stdout/stderr outputed a unvalid utf8";

            let out = String::from_utf8(stdout).expect(msg);
            let err = String::from_utf8(stderr).expect(msg);
            CmdOut {
                stdout: out,
                stderr: err,
            }
        }
    }

    impl fmt::Display for CmdOut {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.stdout.is_empty() {
                let status: String = LogStatus::Info.into();
                let to_print = self.stderr.replace("\n", format!("\n{}", logformat!("", status)).as_str());
                write!(f, "{}", to_print)
            } else {
                write!(f, "{}", self.stdout)
            }
        }
    }


    impl ExecutableCommand<CmdOut> for ExecutableTerminalCommand {
        #[cfg(target_os = "windows")]
        fn _execute(&self) -> Option<CmdOut> {
            let mut cmd = Command::new("cmd");

            let mut serilized = vec!["/C".to_owned()];
            let mut args = self.0.clone();
            serilized.append(&mut args);

            let command = cmd.args(serilized);
            match command.output() {
                Ok(raw_out) => {
                    let out = CmdOut::new(raw_out.stdout, raw_out.stderr);
                    Some(out)
                }
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        fn _execute(&self) -> Option<CmdOut> {
            let mut sh = Command::new("sh");

            let command = sh.args(self.0.clone());
            match command.output() {
                Ok(out) => {
                    let out = CmdOut::new(out.stdout, out.stderr);
                    Some(out)
                },
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
            }
        }
    }

    impl ExecutableCommand<String> for ExacutableFsAction {
        fn _execute(&self) -> Option<String> {
            // checks if the path provided leads to a file
            if self.1 {
                match fs::File::create(&self.0) {
                    Ok(_) => Some(format!("successfuly created file : {}", self.0)),
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            // this else means that if the path is not a file then it is a directory
            } else {
                match fs::create_dir(&self.0) {
                    Ok(()) => Some(format!("successfuly created directory : {}", self.0)),
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            }
        }
    }
    impl CommandSerializer {
        pub fn new(command_name: String, args: Vec<String>, defered: bool) -> Self {
            CommandSerializer {
                _args: args,
                command_name,
                defered,
            }
        }

        pub fn _serialize_to_cmd(&mut self) -> ExecutableTerminalCommand {
            let mut serilized_args = vec![self.command_name.clone()];
            serilized_args.append(&mut self._args);

            ExecutableTerminalCommand(serilized_args, self.defered)
        }

    }
}