use std::collections::{hash_map::Entry, HashMap};

use crate::contracts::{Algorithm, AlgorithmError};
use crate::data_structure::DataStructure;

pub struct TireAlgorithm;

struct TrieNode {
    encode: u8,
    children: HashMap<char, TrieNode>,
}

impl Algorithm for TireAlgorithm {
    fn name(&self) -> &'static str {
        "tire"
    }

    fn description(&self) -> &'static str {
        concat!(
            "A tire, or a prefix tree, is a tree data structure used to encode ",
            "and store strings. Each string will be stored as a leaf node. It ",
            "guarantees the tree path to each leaf is unique and will not be the ",
            "prefix leading to another leaf."
        )
    }

    fn run(&self, _input: &DataStructure) -> Result<DataStructure, AlgorithmError> {
        let DataStructure::StringArray(strings) = _input else {
            return Err(AlgorithmError::invalid_input("string array", _input));
        };

        let mut each_word_prefix = Vec::<String>::new();

        let mut root = TrieNode {
            encode: 0,
            children: HashMap::new(),
        };

        for word in strings {
            let mut head = &mut root;
            for c in word.chars() {
                let next_encode = head.children.len() as u8;
                head = match head.children.entry(c) {
                    Entry::Occupied(entry) => entry.into_mut(),
                    Entry::Vacant(entry) => {
                        println!(
                            "[DEBUG] creating trie node for char {:?} in word {:?} with encode {}",
                            c, word, next_encode
                        );
                        entry.insert(TrieNode {
                            encode: next_encode,
                            children: HashMap::new(),
                        })
                    }
                };
            }
            let leaf_encode = head.children.len() as u8;
            match head.children.entry('\0') {
                Entry::Occupied(_) => {}
                Entry::Vacant(entry) => {
                    println!(
                        "[DEBUG] creating trie leaf for word {:?} with encode {}",
                        word, leaf_encode
                    );
                    entry.insert(TrieNode {
                        encode: leaf_encode,
                        children: HashMap::new(),
                    });
                }
            }
        };

        for word in strings {
            let mut head = & root;
            let mut prefix = Vec::<u8>::new();
            for c in word.chars() {
                let Some(child) = head.children.get(&c) else {
                    return Err(AlgorithmError::new(format!("internal error: unexpected word {word} --- no char {c}")));
                };
                if head.children.len() != 1  {
                    prefix.push(child.encode);
                }
                head = child;
            }
            let Some(leaf) = head.children.get(&'\0') else {
                return Err(AlgorithmError::new(format!("internal error: unexpected word {word} --- not at a leaf node")));
            };
            if head.children.len() != 1  {
                prefix.push(leaf.encode);
            }
            each_word_prefix.push(prefix.iter().map(|e| e.to_string()).collect());
        }

        Ok(DataStructure::StringArray(each_word_prefix))
    }
}
