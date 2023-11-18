/// parses the template files and contians some helper function
/// its main use is to parse the given file into list of string/character
/// which is then passed to the tokenizer
pub mod parser {
    use std::io::ErrorKind::*;
    use std::{fs, io::Read};

    pub struct Parser {
        pub ext: String,
    }

    impl Parser {
        pub fn new(ext: String) -> Self {
            Parser { ext }
        }

        pub fn parse_file(&self, dest: String) -> Result<Vec<String>, std::io::Error> {
            let replaced = dest.replace("\\", "/");
            let files = replaced.split("/").collect::<Vec<&str>>();
            let file_name = files.last().unwrap_or(&"");

            let ext = file_name.split(".").collect::<Vec<&str>>();
            if ext.len() != 2 {
                // occurs when the file has more than one extension (i.e some.file.ext)
                return Err(std::io::Error::new(
                    InvalidInput,
                    "wrong file format make sure the file follows this formate 'file_name.tmplt'",
                ));
            }

            let mut file = fs::File::open(dest)?;
            let mut data = String::new();

            file.read_to_string(&mut data)?;

            let parsed_data = self.parse_data(data);
            Ok(parsed_data)
        }

        fn parse_data(&self, data: String) -> Vec<String> {
            data.split("\n")
                .map(|val| val.trim().to_string())
                .filter(|val| !val.starts_with("//") && !val.is_empty())
                .collect::<Vec<String>>()
        }
    }
}
