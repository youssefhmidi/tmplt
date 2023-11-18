pub mod args {
    use std::{env::args, fmt::Display};

    pub fn get_args() -> Result<Options, ParsingError> {
        let args = args();
        if args.len() == 1 {
            return Err(ParsingError::default())
        }

        let cmd_args_vec: Vec<String> = args.collect();
        let argparser = ArgsParser::new(cmd_args_vec);

        let options = match argparser.parse_options() {
            Ok(some) => some,
            Err(e) => return Err(e),
        };

        Ok(options)
    }

    #[derive(Debug, Clone, Default)]
    pub struct ParsingError {
        reason: String,
    }

    impl ParsingError {
        pub fn no_args(&self) -> bool{
            self.reason.is_empty()
        }
    }

    impl Display for ParsingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f ,"got error while parsing flags/args : {}", self.reason)
        }
    }

    /// contains what was passed with the flag
    ///
    /// # Example
    /// ```
    /// let feature_flag: Flag<bool> = Flag::new(false);
    /// let is_feature_on: &bool = feature_flag.unwrap_ref();
    ///
    /// assert_eq!(is_feature_on, &false)
    /// ```
    #[derive(Debug, Clone, Default)]
    pub struct Flag<T>(T);

    impl<T> Flag<T>
    where
        T: Clone,
    {
        pub fn new(val: T) -> Self {
            Flag(val)
        }

        pub fn unwrap_ref(&self) -> &T {
            &self.0
        }
    }

    #[derive(Debug, Clone, Default)]
    pub enum CommandLineArgs {
        Generate,
        Help,
        New,
        #[default]
        UnknownArg,
        // todo  make it possible to pull other files from a github repo or a url
    }

    #[derive(Debug, Clone)]
    pub enum Flags {
        SaveLogs(bool),
        TaskPerBatch(usize),
        UnknownFlag(ParsingError),
    }

    impl Flags {
        pub fn into_flag<T: Clone>(inner: T) -> Flag<T> {
            Flag::new(inner)
        }
    }

    impl From<String> for Flags {
        fn from(value: String) -> Self {
            match value.as_str() {
                str if str.starts_with("--task-num") || str.starts_with("--batch-size") => {
                    let flag = str
                        .split("=")
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>();
                    if flag.len() == 1 {
                        return Self::UnknownFlag(ParsingError { reason : "misused the batch size flag, example'--batch-size=50'".to_string()});
                    }

                    let num = match flag[1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => return Self::UnknownFlag(ParsingError { reason : "should've passed a number after the '='".to_string()}),
                    };

                    Self::TaskPerBatch(num)
                },
                "--sl" | "--save-logs" | "--logged" => Self::SaveLogs(true),
                &_ => Self::UnknownFlag(ParsingError { reason : "unknow flag has been passed, to check all the avaliable flags use 'tmplt help' and check the generate command section".to_string()}),
            }
        }
    }

    impl From<String> for CommandLineArgs {
        fn from(value: String) -> Self {
            match value.as_str() {
                "gen" | "generate" | "g" => Self::Generate,
                "help" | "h" => Self::Help,
                "init" | "new" => Self::New,
                &_ => Self::UnknownArg,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ArgsParser {
        args: Vec<String>,
        flags: Vec<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct Options {
        command: CommandLineArgs,
        file: Option<String>,
        save_logs: Flag<bool>,
        task_num: Flag<usize>,
    }

    impl Options {
        pub fn get_command(&self) -> CommandLineArgs {
            self.command.clone()
        }

        pub fn get_file(&self) -> Option<String> {
            self.file.clone()
        }

        pub fn get_save_logs_flag(&self) -> &bool {
            self.save_logs.unwrap_ref()
        }

        pub fn get_batch_size_flag(&self) -> &usize {
            let size = self.task_num.unwrap_ref();
            if *size == 0 {
                return &10;
            } else {
                size
            }
        }
    }

    impl ArgsParser {
        pub fn new(mut args: Vec<String>) -> Self {
            let flags = args
                .iter()
                .filter(|v| v.starts_with("--"))
                .map(|v| v.to_owned())
                .collect();
            args.remove(0);
            let args: Vec<String> = args
                .iter()
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .collect();

            ArgsParser { args, flags }
        }

        pub fn parse_options(&self) -> Result<Options, ParsingError> {
            let file = self
                .args
                .iter()
                .filter(|v| v.contains(".tmplt"))
                .cloned()
                .collect::<Vec<String>>();
            let mut flags_vec: Vec<Flags> = vec![];

            for f in self.flags.clone() {
                let flag = Flags::from(f);
                flags_vec.push(flag)
            }
            let mut options: Options = Options::default();
            options.file = None;

            if file.len() != 0 {
                options.file = Some(file[0].clone());
            }

            options.command = CommandLineArgs::from(self.args[0].clone());

            for flag in flags_vec {
                match flag {
                    Flags::SaveLogs(inner) => options.save_logs = Flags::into_flag(inner),
                    Flags::TaskPerBatch(inner) => options.task_num = Flags::into_flag(inner),
                    Flags::UnknownFlag(error) => return Err(error),
                }
            }

            Ok(options)
        }
    }
}
