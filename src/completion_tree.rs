use std::collections::{BTreeMap, BTreeSet};
use std::{rc::Rc, str::Chars};

/// A completion tree that holds and handles completions
#[derive(Debug, Clone)]
pub struct CompletionTree {
    root: CompletionNode,
    inclusions: Rc<BTreeSet<char>>,
    min_word_len: usize,
}

impl Default for CompletionTree {
    fn default() -> Self {
        let inclusions = Rc::new(BTreeSet::new());
        Self {
            root: CompletionNode::new(inclusions.clone()),
            inclusions,
            min_word_len: 5,
        }
    }
}

impl CompletionTree {
    /// Create a new CompletionTree with provided non alphabet characters whitelisted.
    /// The default CompletionTree will only parse alphabet characters (a-z, A-Z). Use this to
    /// introduce additional accepted special characters.
    ///
    /// # Arguments
    ///
    /// * `incl`    An array slice with allowed characters
    ///
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.insert("test-hyphen test_underscore");
    /// assert_eq!(
    ///     completions.complete("te"),
    ///     Some(vec!["test".to_string()]));
    ///
    /// let mut completions = CompletionTree::with_inclusions(&['-', '_']);
    /// completions.insert("test-hyphen test_underscore");
    /// assert_eq!(
    ///     completions.complete("te"),
    ///     Some(vec!["test-hyphen".to_string(), "test_underscore".to_string()]));
    /// ```
    pub fn with_inclusions(incl: &[char]) -> Self {
        let mut set = BTreeSet::new();
        incl.iter().for_each(|c| {
            set.insert(*c);
        });
        let inclusions = Rc::new(set);
        Self {
            root: CompletionNode::new(inclusions.clone()),
            inclusions,
            ..Self::default()
        }
    }

    /// Inserts one or more words into the completion tree for later use.
    /// Input is automatically split on 'whitespace' using String::split_whitespace().
    ///
    /// # Arguments
    ///
    /// * `line`    A str slice containing one or more words
    ///
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    ///
    /// // Insert multiple words
    /// completions.insert("a line with many words");
    ///
    /// // The above line is equal to the following:
    /// completions.insert("a");
    /// completions.insert("line");
    /// completions.insert("with");
    /// completions.insert("many");
    /// completions.insert("words");
    /// ```
    pub fn insert(&mut self, line: &str) {
        for word in line.split_whitespace() {
            if word.len() >= self.min_word_len {
                self.root.insert(word.chars());
            }
        }
    }

    /// Returns an optional vector of completions based on the provided input
    ///
    /// # Arguments
    ///
    /// * `line`    The line to complete
    ///             In case of multiple words, only the last will be completed
    ///
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.insert("batman robin batmobile batcave robber");
    /// assert_eq!(
    ///     completions.complete("bat"),
    ///     Some(vec!["batcave", "batman", "batmobile"].iter().map(|s| s.to_string()).collect()));
    ///
    /// assert_eq!(
    ///     completions.complete("to the bat"),
    ///     Some(vec!["to the batcave", "to the batman", "to the batmobile"].iter().map(|s| s.to_string()).collect()));
    /// ```
    pub fn complete(&self, line: &str) -> Option<Vec<String>> {
        if !line.is_empty() {
            let last_word = line.split_whitespace().last().unwrap();
            if let Some(mut extensions) = self.root.complete(last_word.chars()) {
                extensions.sort();
                return Some(
                    extensions
                        .iter()
                        .map(|ext| format!("{}{}", line, ext))
                        .collect::<Vec<String>>(),
                );
            } else {
                return None;
            }
        }
        None
    }

    /// Clears all the data from the tree
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.insert("batman robin batmobile batcave robber");
    /// assert_eq!(completions.word_count(), 5);
    /// assert_eq!(completions.size(), 24);
    /// completions.clear();
    /// assert_eq!(completions.size(), 1);
    /// assert_eq!(completions.word_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.root.clear();
    }

    /// Returns a count of how many words that exist in the tree
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.insert("batman robin batmobile batcave robber");
    /// assert_eq!(completions.word_count(), 5);
    /// ```
    pub fn word_count(&self) -> u32 {
        self.root.word_count()
    }

    /// Returns the size of the tree, the amount of nodes, not words
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.insert("batman robin batmobile batcave robber");
    /// assert_eq!(completions.size(), 24);
    /// ```
    pub fn size(&self) -> u32 {
        self.root.subnode_count()
    }

    /// Returns the minimum word length to complete. This allows you
    /// to pass full sentences to `insert()` and not worry about
    /// pruning out small words like "a" or "to", because they will be
    /// ignored.
    /// # Example
    /// ```
    /// extern crate rs_complete;
    /// use rs_complete::CompletionTree;
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.set_min_word_len(4);
    /// completions.insert("one two three four five");
    /// assert_eq!(completions.word_count(), 3);
    ///
    /// let mut completions = CompletionTree::default();
    /// completions.set_min_word_len(1);
    /// completions.insert("one two three four five");
    /// assert_eq!(completions.word_count(), 5);
    /// ```
    pub fn min_word_len(&self) -> usize {
        self.min_word_len
    }

    /// Sets the minimum word length to complete on. Smaller words are
    /// ignored. This only affects future calls to `insert()` -
    /// changing this won't start completing on smaller words that
    /// were added in the past, nor will it exclude larger words
    /// already inserted into the completion tree.
    pub fn set_min_word_len(&mut self, len: usize) {
        self.min_word_len = len;
    }
}

#[derive(Debug, Clone)]
struct CompletionNode {
    subnodes: BTreeMap<char, CompletionNode>,
    leaf: bool,
    inclusions: Rc<BTreeSet<char>>,
}

impl CompletionNode {
    fn new(incl: Rc<BTreeSet<char>>) -> Self {
        Self {
            subnodes: BTreeMap::new(),
            leaf: false,
            inclusions: incl,
        }
    }

    fn clear(&mut self) {
        self.subnodes.clear();
    }

    fn word_count(&self) -> u32 {
        let mut count = self.subnodes.values().map(|n| n.word_count()).sum();
        if self.leaf {
            count += 1;
        }
        count
    }

    fn subnode_count(&self) -> u32 {
        self.subnodes
            .values()
            .map(|n| n.subnode_count())
            .sum::<u32>()
            + 1
    }

    fn insert(&mut self, mut iter: Chars) {
        if let Some(c) = iter.next() {
            if self.inclusions.contains(&c) || c.is_alphanumeric() {
                let inclusions = self.inclusions.clone();
                let subnode = self
                    .subnodes
                    .entry(c)
                    .or_insert_with(|| CompletionNode::new(inclusions));
                subnode.insert(iter);
            } else {
                self.leaf = true;
            }
        } else {
            self.leaf = true;
        }
    }

    fn complete(&self, mut iter: Chars) -> Option<Vec<String>> {
        if let Some(c) = iter.next() {
            if let Some(subnode) = self.subnodes.get(&c) {
                subnode.complete(iter)
            } else {
                None
            }
        } else {
            Some(self.collect("".to_string()))
        }
    }

    fn collect(&self, partial: String) -> Vec<String> {
        let mut completions = vec![];
        if self.leaf {
            completions.push(partial.clone());
        }

        if !self.subnodes.is_empty() {
            for (c, node) in &self.subnodes {
                let mut partial = partial.clone();
                partial.push(*c);
                completions.append(&mut node.collect(partial));
            }
        }
        completions
    }
}
