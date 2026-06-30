use std::fmt::{Display, Formatter};

pub(crate) mod graph;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataStructure {
    RawString(String),
    StringArray(Vec<String>),
    Graph(Vec<graph::NodeRef>),
}

#[allow(dead_code)]
impl DataStructure {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::RawString(_) => "raw-string",
            Self::StringArray(_) => "string-array",
            Self::Graph(_) => "graph",
        }
    }
}

impl Display for DataStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawString(raw_string) => write!(f, "{raw_string}"),
            Self::StringArray(strings) => {
                write!(f, "[")?;

                for (index, value) in strings.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "\"{value}\"")?;
                }

                write!(f, "]")
            },
            Self::Graph(nodes, ) => graph::display_graph(f, nodes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataStructure;

    #[test]
    fn exposes_raw_string_helpers() {
        let value = DataStructure::RawString("hello".to_string());

        assert_eq!(value.kind(), "raw-string");

        let DataStructure::RawString(raw_string) = value else {
            panic!("expected raw-string");
        };
        assert_eq!(raw_string, "hello");
    }

    #[test]
    fn exposes_string_array_helpers() {
        let value = DataStructure::StringArray(vec!["cat".to_string(), "car".to_string()]);

        assert_eq!(value.kind(), "string-array");
        let DataStructure::StringArray(strings) = value else {
            panic!("expected string array");
        };
        assert_eq!(strings, vec!["cat".to_string(), "car".to_string()]);
    }

    #[test]
    fn displays_raw_string_without_extra_wrapping() {
        let value = DataStructure::RawString("hello".to_string());

        assert_eq!(value.to_string(), "hello");
    }

    #[test]
    fn displays_string_array_as_quoted_items() {
        let value = DataStructure::StringArray(vec!["1".to_string(), "2".to_string()]);

        assert_eq!(value.to_string(), "[\"1\", \"2\"]");
    }
}
