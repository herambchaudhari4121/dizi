use std::cmp;
use std::collections::VecDeque;
use std::fs;
use std::time;

use serde_derive::Deserialize;

use crate::fs::DirEntry;
use crate::util::sort_type::{SortType, SortTypes};

#[derive(Clone, Debug)]
pub struct SortOption {
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_methods: SortTypes,
}

impl SortOption {
    pub fn set_sort_method(&mut self, method: SortType) {
        self.sort_methods.reorganize(method);
    }

    pub fn compare(&self, f1: &DirEntry, f2: &DirEntry) -> cmp::Ordering {
        if self.directories_first {
            let f1_isdir = f1.file_path().is_dir();
            let f2_isdir = f2.file_path().is_dir();

            if f1_isdir && !f2_isdir {
                return cmp::Ordering::Less;
            } else if !f1_isdir && f2_isdir {
                return cmp::Ordering::Greater;
            }
        }

        // let mut res = self.sort_method.cmp(f1, f2, &self);
        let mut res = self.sort_methods.cmp(f1, f2, &self);
        if self.reverse {
            res = match res {
                cmp::Ordering::Less => cmp::Ordering::Greater,
                cmp::Ordering::Greater => cmp::Ordering::Less,
                s => s,
            };
        };
        res
    }
}

impl std::default::Default for SortOption {
    fn default() -> Self {
        SortOption {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_methods: SortTypes::default(),
        }
    }
}
