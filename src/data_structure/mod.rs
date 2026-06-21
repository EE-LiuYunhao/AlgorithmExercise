#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataStructure {
    RawString(String),
    StringArray(Vec<String>),
}

#[allow(dead_code)]
impl DataStructure {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::RawString(_) => "raw-string",
            Self::StringArray(_) => "string-array",
        }
    }

    pub fn as_raw_string(&self) -> Option<&str> {
        match self {
            Self::RawString(value) => Some(value.as_str()),
            Self::StringArray(_) => None,
        }
    }

    pub fn as_string_array(&self) -> Option<&[String]> {
        match self {
            Self::RawString(_) => None,
            Self::StringArray(values) => Some(values.as_slice()),
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
        assert_eq!(value.as_raw_string(), Some("hello"));
        assert_eq!(value.as_string_array(), None);
    }

    #[test]
    fn exposes_string_array_helpers() {
        let value = DataStructure::StringArray(vec!["cat".to_string(), "car".to_string()]);

        assert_eq!(value.kind(), "string-array");
        assert_eq!(value.as_raw_string(), None);
        assert_eq!(
            value.as_string_array(),
            Some(vec!["cat".to_string(), "car".to_string()].as_slice())
        );
    }
}
