use std::borrow::Cow;

use multimap::MultiMap;

/// Parsed Arguments
pub struct Arguments {
    arg_map: MultiMap<String, Option<String>>,
}

impl Arguments {
    /// Parse arguments. This cannot fail. Flags are simply
    /// denoted by a single `-` followed by the flag,
    /// and the value is immediately after. Multiple instances
    /// can be contained, and flags can contain no value
    ///
    /// # Arguments
    ///
    /// `args`: The arguments
    pub fn parse<'a>(args: Cow<'a, [&'a str]>) -> Arguments {
        let mut arg_map = MultiMap::new();
        for (&key, &val) in args
            .iter()
            .zip(args.iter().skip(1).chain(std::iter::once(&"")))
        {
            if key.starts_with('-') {
                arg_map.insert(
                    key[1..].to_string(),
                    if val.is_empty() || val.starts_with('-') {
                        None
                    } else {
                        Some(val.to_string())
                    },
                );
            }
        }
        Arguments { arg_map }
    }

    /// Checks whether or not an argument is present in the list
    ///
    /// # Arguments
    ///
    /// `key`: The key to check
    pub fn contains(&self, key: &str) -> bool {
        self.arg_map.contains_key(key)
    }

    /// Checks whether or not an argument is present in the list
    /// with a non-empty value
    ///
    /// # Arguments
    ///
    /// `key`: The key to check
    pub fn contains_val(&self, key: &str) -> bool {
        self.arg_map
            .get_vec(key)
            .and_then(|vals| vals.iter().find(|&val| val.is_some()))
            .is_some()
    }

    /// Checks whether or not the arguments are empty
    pub fn is_empty(&self) -> bool {
        self.arg_map.is_empty()
    }

    /// Gets the first value with the given key
    ///
    /// # Arguments
    ///
    /// `key`: The key to fetch
    pub fn get(&self, key: &str) -> Option<&Option<String>> {
        self.arg_map.get(key)
    }

    /// Gets all values with the given key
    ///
    /// # Arguments
    ///
    /// `key`: The key to fetch
    pub fn get_vec(&self, key: &str) -> Option<&Vec<Option<String>>> {
        self.arg_map.get_vec(key)
    }

    /// Returns the number of arguments that were parsed
    pub fn len(&self) -> usize {
        self.arg_map.len()
    }
}

#[cfg(test)]
mod test {
    use super::Arguments;
    use std::borrow::Cow;

    #[test]
    fn empty() {
        let args = Arguments::parse(Cow::Borrowed(&[]));
        assert!(args.is_empty());
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn no_arg() {
        let args = Arguments::parse(Cow::Borrowed(&["arg"]));
        assert!(args.is_empty());
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn one_empty() {
        let args = Arguments::parse(Cow::Borrowed(&["-key"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 1);
        assert!(args.contains("key"));
        assert!(!args.contains_val("key"));
        assert!(args.get("key").is_some());
        assert_eq!(args.get_vec("key").unwrap().len(), 1);
    }

    #[test]
    fn one_key() {
        let args = Arguments::parse(Cow::Borrowed(&["-key", "val"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 1);
        assert!(args.contains("key"));
        assert!(args.contains_val("key"));
        assert!(args.get("key").is_some());
        assert_eq!(args.get("key").unwrap().as_ref().unwrap(), "val");
        assert_eq!(args.get_vec("key").unwrap().len(), 1);
    }

    #[test]
    fn one_key_repeated() {
        let args = Arguments::parse(Cow::Borrowed(&["-key", "val", "-key", "val2"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 1);
        assert!(args.contains("key"));
        assert!(args.contains_val("key"));
        assert!(args.get("key").is_some());
        assert_eq!(args.get("key").unwrap().as_ref().unwrap(), "val");
        assert_eq!(args.get_vec("key").unwrap(), &vec!(Some("val".to_string()), Some("val2".to_string())));
        assert_eq!(args.get_vec("key").unwrap().len(), 2);
    }

    #[test]
    fn one_key_cut_short() {
        let args = Arguments::parse(Cow::Borrowed(&["-key", "-key", "val2"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 1);
        assert!(args.contains("key"));
        assert!(args.contains_val("key"));
        assert!(args.get("key").is_some());
        assert_eq!(args.get("key").unwrap(), &None);
        assert_eq!(args.get_vec("key").unwrap(), &vec!(None, Some("val2".to_string())));
        assert_eq!(args.get_vec("key").unwrap().len(), 2);
    }

    #[test]
    fn two_keys() {
        let args = Arguments::parse(Cow::Borrowed(&["-key", "val", "-key2", "val2"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 2);
        assert!(args.contains("key"));
        assert!(args.contains("key2"));
        assert!(args.contains_val("key"));
        assert!(args.contains_val("key2"));
        assert!(args.get("key").is_some());
        assert!(args.get("key2").is_some());
        assert_eq!(args.get("key").unwrap().as_ref().unwrap(), "val");
        assert_eq!(args.get("key2").unwrap().as_ref().unwrap(), "val2");
        assert_eq!(args.get_vec("key").unwrap().len(), 1);
        assert_eq!(args.get_vec("key2").unwrap().len(), 1);
    }

    #[test]
    fn two_keys_cut_short() {
        let args = Arguments::parse(Cow::Borrowed(&["-key", "-key2", "val2"]));
        assert!(!args.is_empty());
        assert_eq!(args.len(), 2);
        assert!(args.contains("key"));
        assert!(args.contains("key2"));
        assert!(!args.contains_val("key"));
        assert!(args.contains_val("key2"));
        assert!(args.get("key").is_some());
        assert!(args.get("key2").is_some());
        assert_eq!(args.get("key").unwrap(), &None);
        assert_eq!(args.get("key2").unwrap().as_ref().unwrap(), "val2");
        assert_eq!(args.get_vec("key").unwrap().len(), 1);
        assert_eq!(args.get_vec("key2").unwrap().len(), 1);
    }
}