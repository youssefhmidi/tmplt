pub mod writer {
    use chrono::{DateTime, Local};
    use std::{fs::File, io::Write, path::Path};
    use std::env;

    #[derive(Clone, Debug)]
    pub enum LogStatus {
        Info,
        Warning,
        Error,
        ForcedAction,
    }

    impl Into<String> for LogStatus {
        fn into(self) -> String {
            match self {
                LogStatus::Info => "INFO".to_string(),
                LogStatus::Warning => "WARNING".to_string(),
                LogStatus::Error => "ERROR".to_string(),
                LogStatus::ForcedAction => "FORCED".to_string(),
            }
        }
    }

    #[derive(Debug)]
    pub struct LogWriter {
        timestamp: DateTime<Local>,
        log_prefix: String,

        lines_buffer: Vec<String>,
        file_buf: Option<File>,
    }

    impl LogWriter {
        pub fn new(logs_prefix: String) -> Self {
            let time = Local::now();

            LogWriter {
                timestamp: time,
                log_prefix: logs_prefix,
                file_buf: None,
                lines_buffer: vec![]
            }
        }

        pub fn get_file(&self) -> Option<&File> {
            self.file_buf.as_ref()
        }

        pub fn get_prefix(&self) -> String {
            self.log_prefix.clone()
        }

        pub fn initialize_logger(logs_prefix: String) -> Self {
            let mut logger = LogWriter::new(logs_prefix);

            logger.init_file();

            logger
        }

        pub fn write(&mut self, text: String, status: LogStatus) {
            let line = format!(
                "[{} | {}] > {}",
                self.log_prefix,
                <LogStatus as Into<String>>::into(status),
                text
            );

            self.lines_buffer.push(line)
        }

        pub fn write_to_file(&self) {
            let mut file = match self.get_file() {
                Some(f) => f,
                None => return eprintln!("cannot find file"),
            };

            match file.write_all(self.lines_buffer.join("\n").as_bytes()) {
                Ok(_) => return,
                Err(e) => return eprintln!("couldn't write to file, got error: {}", e),
            };
        }

        fn init_file(&mut self) {
            match self.file_buf {
                Some(_) => (),
                None => {
                    let exe_dir = env::current_exe().unwrap();
                    let parent = exe_dir.parent().unwrap();
                    let path = format!("{}\\..\\logs\\{}.log", parent.display(), self.formated_timestamp(parent.display().to_string()));
                    let file = match File::create(path){
                        Ok(f) => f,
                        Err(e) => return eprintln!("got err while creating log file, this error shouldn't appear, consider opening an issue in the github repository.\n Error: {}", e)
                    };

                    self.file_buf = Some(file)
                }
            }
        }

        fn exists(&self, path: String) -> bool {
            let is_exists = Path::new(&path).exists();

            is_exists
        }

        fn formated_timestamp(&self, parent: String) -> String {
            let mut last_number: usize = 0;

            let formated_name = loop {
                let string = if last_number != 0 {
                    format!("{}_{last_number}", self.timestamp.format("%d-%m-%Y"))
                } else {
                    format!("{}", self.timestamp.format("%d-%m-%Y"))
                };


                if self.exists(format!("{parent}\\..\\logs\\{}.log", string)) {
                    last_number += 1
                } else {
                    break string;
                }
            };

            formated_name
        }
    }
}
