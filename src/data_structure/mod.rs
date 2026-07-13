use std::fmt::{Display, Formatter};

pub(crate) mod graph;
pub(crate) mod linked_list;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Shared payload type exchanged between parsers and algorithms.
///
/// Each variant represents one supported structured input or output shape used
/// by the CLI runtime.
pub(crate) enum DataStructure {
    RawString(String),
    Int(i32),
    IntArray(Vec<i32>),
    StringArray(Vec<String>),
    Graph(Vec<graph::NodeRef>),
    LinkedListInt(linked_list::NodeRef<i32>),
    LinkedListString(linked_list::NodeRef<String>),
    LinkedListFloat(linked_list::NodeRef<f32>),
}

#[allow(dead_code)]
impl DataStructure {
    /// Returns the stable kind name used in diagnostics and error messages.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::RawString(_) => "raw-string",
            Self::Int(_) => "int",
            Self::IntArray(_) => "int-array",
            Self::StringArray(_) => "string-array",
            Self::Graph(_) => "graph",
            Self::LinkedListInt(_) => "linked-list(int32)",
            Self::LinkedListFloat(_) => "linked-list(float32)",
            Self::LinkedListString(_) => "linked-list(string)",
        }
    }
}

impl Display for DataStructure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawString(raw_string) => write!(f, "{raw_string}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::IntArray(ints) => {
                write!(f, "[")?;
                for (index, value) in ints.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "\"{value}\"")?;
                }

                write!(f, "]")
            }
            Self::StringArray(strings) => {
                write!(f, "[")?;

                for (index, value) in strings.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "\"{value}\"")?;
                }

                write!(f, "]")
            }
            Self::Graph(nodes) => graph::display(f, nodes),
            Self::LinkedListInt(head) => linked_list::display::<i32>(f, head),
            Self::LinkedListFloat(head) => linked_list::display::<f32>(f, head),
            Self::LinkedListString(head) => linked_list::display::<String>(f, head),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataStructure;

    fn assert_eq_impl<T: Eq>() {}

    #[test]
    fn data_structure_implements_eq_with_linked_list_float_variant_present() {
        assert_eq_impl::<DataStructure>();
    }

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
