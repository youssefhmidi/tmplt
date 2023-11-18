pub mod syntax_tree {
    use core::fmt;

    use crate::core::Tokens::*;

    /// the expected Tree should have three branches and every branch may have
    /// multpile nodes
    ///
    /// tree: the entire file
    /// branch: diffrent sections (for example __CWD is a section)
    /// node: a line (for example 'DEFER npm i' is a node)
    #[derive(Clone, Debug)]
    pub struct Tree {
        _file_vec: Vec<String>,

        pub branches: Vec<Branch>,
    }

    /// a branch have a specified name and contains all the lines that need to be executed
    ///
    /// nodes represents lines
    ///
    /// Note : the Branch struct act as a sections sperator so it will help to contribute task to implement asynchronous task execution
    #[derive(Clone, Debug)]
    pub struct Branch {
        _section_vec: Vec<String>,

        pub section_kind: SectionIdentity,
        pub nodes: Vec<Node>,
    }

    /// a Node is a strut that contains the entire line and a current_token that can be iterable
    /// to access the Toke next to it
    #[derive(Clone, Debug)]
    pub struct Node {
        curren_tk_idx: usize,
        curent_iteration: usize,
        words: Vec<String>,

        pub current_token: Token,
        pub text: String,
    }

    impl Tree {
        pub fn construct(file_data: Vec<String>) -> Result<Self, UnknownSectionError> {
            let mut branches: Vec<Branch> = Vec::with_capacity(3);

            for (idx, line) in file_data.iter().enumerate() {
                if line.starts_with("__") {
                    let sect_name = line.split(" ").collect::<Vec<&str>>()[0];

                    let lines = file_data
                        .iter()
                        .skip(idx + 1)
                        .take_while(|val| !val.starts_with("__"))
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>();

                    let sect = SectionIdentity::from(sect_name.to_string());
                    match sect.unknown(sect_name.to_string()) {
                        Ok(_) => branches.push(self::Branch::construct(lines, sect)),
                        Err(e) => return Err(e),
                    }
                }
            }

            Ok(Tree {
                _file_vec: file_data,
                branches,
            })
        }
    }

    impl fmt::Display for Tree {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let branches = self
                .branches
                .iter()
                // formats for every branch a taxt contains the kind of the branch and the nodes containing it
                .map(|v| {
                    let nodes = v.nodes.clone();

                    // formats for every nodes a text displaying the line
                    let text = nodes
                        .iter()
                        .map(|n| n.words.join(" "))
                        .collect::<Vec<String>>()
                        .join("\n        --- ");

                    format!(
                        "\n      -- {:?} Nodes : \n        --- {text}",
                        v.section_kind
                    )
                })
                .collect::<Vec<String>>();
            let text = format!(
                "   Syntax Tree :\n{} 
                ",
                branches.join("\n")
            );

            write!(f, "{text}")
        }
    }

    impl Branch {
        pub fn construct(section: Vec<String>, section_kind: SectionIdentity) -> Self {
            let nodes = section
                .iter()
                .map(|v| Node::new(v.to_string()))
                .collect::<Vec<Node>>();

            Branch {
                _section_vec: section,
                section_kind,
                nodes,
            }
        }
    }

    impl Node {
        pub fn new(line: String) -> Self {
            let first_keyword = line.split(" ").next().unwrap().to_string();

            let tkn = Token::from(first_keyword.clone());

            Node {
                words: line
                    .split(" ")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>(),
                curren_tk_idx: 0,
                curent_iteration: 0,
                current_token: tkn,
                text: first_keyword,
            }
        }

        /// get_words is a helper function the access the words field in Node without modifing it.
        ///
        /// the way the `Iterator` trait is implemented for the `Node` struct rely on the `words` filed to determine the next word
        /// if (for example) you decide to remove a word or to filter the words list, the `Node` will break down
        /// by skipping some words which destroys the scripts/ commands that you specified in the `.tmplt` file
        pub fn get_words(&self) -> Vec<String> {
            self.words.clone()
        }
    }

    impl Iterator for Node {
        type Item = (Token, String);

        fn next(&mut self) -> Option<Self::Item> {
            // checks if there is no item left
            if self.curren_tk_idx == self.words.len() - 1 {
                return None;
            }

            // checks if the iteration just started
            if self.curent_iteration == 0 {
                self.curent_iteration += 1;

                return Some((self.current_token.clone(), self.text.clone()));
            }

            // does normal code
            self.curent_iteration += 1;
            self.curren_tk_idx += 1;

            let text = self.words[self.curren_tk_idx].clone();
            let tkn = Token::from(text.to_string());

            self.current_token = tkn.clone();
            self.text = text.to_string();

            Some((tkn, text.to_string()))
        }
    }
}
