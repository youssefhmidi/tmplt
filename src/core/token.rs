/// module for identifying keywords,
/// this module is often used to construct the syntax tree
pub mod token {
    use core::fmt;

    use SectionIdentity::*;
    use Token::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Token {
        DeclareFolder,
        DeclareFile,
        CopyAction,
        Assign,
        DeferAction,
        Arg(String),
        /// used mainly for storing texts and scripts in the current moment but may have multiple uses (i.e ordered commands)
        /// Note that this is the first ever version so I wont go crazy with it
        Skip,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SectionIdentity {
        CurrentWorkingDirectory,
        Demostration,
        Scripts,
        Variables,
        UnknownSection,
    }

    #[derive(Clone, Debug)]
    pub struct UnknownSectionError {
        at: String,
    }

    impl fmt::Display for UnknownSectionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Unknown section name : '{}' ", self.at)
        }
    }

    impl From<String> for Token {
        fn from(value: String) -> Self {
            match value.as_str() {
                "FOLDER" | "FLDR" => DeclareFolder,
                "FILE" => DeclareFile,
                "COPY" | "COPY_INTO" => CopyAction,
                "DEFER" => DeferAction,
                "=" => Assign,
                str if str.starts_with("#") => Arg(str.to_string()),
                &_ => Skip,
            }
        }
    }

    impl From<String> for SectionIdentity {
        fn from(value: String) -> Self {
            match value.as_str() {
                "__CWD" | "__FILE_STRUCT" => CurrentWorkingDirectory,
                "__DEMO" | "__EXAMPLES" => Demostration,
                "__SCRIPTS" | "__CMD" => Scripts,
                "__VAR" | "__VARIABLES" | "__ARGS" => Variables,

                &_ => UnknownSection,
            }
        }
    }

    impl SectionIdentity {
        pub fn unknown(&self, at: String) -> Result<(), UnknownSectionError> {
            match self {
                UnknownSection => Err(UnknownSectionError { at }),
                _ => Ok(()),
            }
        }
    }
}
