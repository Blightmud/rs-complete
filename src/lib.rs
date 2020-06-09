//! rs-completion is a library to use when you want to implement tab-completion (or similar)
//! in your project.
//!
//! rs-completion is mainly built for memory efficiency. Completions are stored in binary trees
//! where each node holds a number of characters. The characters in turn link to new nodes. Similar
//! words will thusly share memory.
//!
//! ## Visual example
//!
//! ```
//! //                          'c' - 'a' - 'v' - 'e'
//! //                         /
//! //    root - 'b' - 'a' - 't' - 'm' - 'a' - 'n'
//! //                               \
//! //                                'o' - 'b' - 'i' - 'l' - 'e'
//! ```
//!
//!
//! This means that a worst case scenario you could have 25^25 nodes in memory where 25 is the size
//! of your alphabet. But this would mean that you are holding every thinkable combination of
//! characters in memory with no regards for consonant or verb rules. If this is what you need then
//! you don't need a library for it.
//!
//! I can't argue if this solution is fast or efficient. It has worked to solve the problem 
//! I intended to solve when I created the library. If you have ideas for extensions or
//! improvements I'm happy to see them.
//!
//! ## Example
//! ```
//! extern crate rs_complete;
//! use rs_complete::CompletionTree;
//!
//! let mut completions = CompletionTree::default();
//!
//! completions.insert("large bunch of words that bungalow we want to be bundesliga able to complete");
//! assert_eq!(
//!     completions.complete("bun"),
//!     Some(vec!["bunch", "bundesliga", "bungalow"].iter().map(|s| s.to_string()).collect()));
//! ```

#[allow(dead_code)]
mod completion_tree;

pub use completion_tree::CompletionTree;

#[cfg(test)]
mod tests {
    use crate::completion_tree::CompletionTree;

    #[test]
    fn test_completion() {
        let mut tree = CompletionTree::default();
        tree.insert("wording");
        let completions = tree.complete("wo").unwrap_or(vec![]);
        let mut iter = completions.iter();
        assert_eq!(iter.next(), Some(&"wording".to_string()));
    }

    #[test]
    fn test_multi_completion() {
        let mut tree = CompletionTree::default();
        tree.insert("wording");
        tree.insert("wollybugger");
        tree.insert("workerbee");
        tree.insert("worldleader");
        tree.insert("batman");
        tree.insert("robin");
        let completions = tree.complete("wo").unwrap();
        assert!(completions.contains(&"workerbee".to_string()));
        assert!(completions.contains(&"wollybugger".to_string()));
        assert!(completions.contains(&"wording".to_string()));
        assert!(completions.contains(&"worldleader".to_string()));
        assert!(!completions.contains(&"batman".to_string()));
        assert!(!completions.contains(&"robin".to_string()));
    }

    #[test]
    fn test_multi_insert() {
        let mut tree = CompletionTree::default();
        tree.insert("wollybugger workerbee worldleader batman robin wording");
        let completions = tree.complete("wo").unwrap();
        assert!(completions.contains(&"workerbee".to_string()));
        assert!(completions.contains(&"wollybugger".to_string()));
        assert!(completions.contains(&"wording".to_string()));
        assert!(completions.contains(&"worldleader".to_string()));
        assert!(!completions.contains(&"batman".to_string()));
        assert!(!completions.contains(&"robin".to_string()));
    }

    #[test]
    fn test_substring_matches() {
        let mut tree = CompletionTree::default();
        tree.insert("dumpster dumpsterfire");
        let completions = tree.complete("dum").unwrap();
        assert!(completions.contains(&"dumpster".to_string()));
        assert!(completions.contains(&"dumpsterfire".to_string()));
    }

    #[test]
    fn test_dont_include_specials() {
        let mut tree = CompletionTree::default();
        tree.insert("dumpster\x1b[34m dumpsterfire{}");
        let completions = tree.complete("dum").unwrap();
        assert!(completions.contains(&"dumpster".to_string()));
        assert!(completions.contains(&"dumpsterfire".to_string()));
    }

    #[test]
    fn test_without_inclusions() {
        let mut tree = CompletionTree::default();
        tree.insert("/dumpster /dumpsterfire");
        assert!(tree.complete("/dum").is_none());
    }

    #[test]
    fn test_with_inclusions() {
        let mut tree = CompletionTree::with_inclusions(&['/', '_']);
        tree.insert("/dumpster /dumpster_fire");
        let completions = tree.complete("/dum").unwrap();
        assert!(completions.contains(&"/dumpster".to_string()));
        assert!(completions.contains(&"/dumpster_fire".to_string()));
    }
}
