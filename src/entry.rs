// Conserve backup system.
// Copyright 2015, 2016, 2017, 2018, 2019, 2020 Martin Pool.

//! An entry representing a file, directory, etc, in either a
//! stored tree or local tree.

use std::fmt::Debug;

use crate::kind::Kind;
use crate::unix_time::UnixTime;
use crate::*;

pub trait Entry: Debug + Eq + PartialEq {
    fn apath(&self) -> &Apath;
    fn kind(&self) -> Kind;
    fn mtime(&self) -> UnixTime;
    fn size(&self) -> Option<u64>;
    fn symlink_target(&self) -> &Option<String>;

    /// True if the metadata supports an assumption the file contents have
    /// not changed.
    fn is_unchanged_from<O: Entry>(&self, basis_entry: &O) -> bool {
        basis_entry.kind() == self.kind()
            && basis_entry.mtime() == self.mtime()
            && basis_entry.size() == self.size()
    }
}
