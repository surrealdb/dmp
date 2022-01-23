# dmp

The Diff Match and Patch libraries offer robust algorithms to perform the operations required for synchronising plain text. This repository contains a Rust version of the original [diff-match-patch](https://github.com/google/diff-match-patch) library, using up-to-date crate packages.

[![](https://img.shields.io/badge/status-alpha-ff00bb.svg?style=flat-square)](https://github.com/surrealdb/dmp) ![docs.rs](https://img.shields.io/docsrs/dmp?style=flat-square) ![Crates.io](https://img.shields.io/crates/v/dmp?style=flat-square) [![](https://img.shields.io/badge/license-MIT-00bfff.svg?style=flat-square)](https://github.com/surrealdb/dmp) 

#### Features

- Diffing and patching library for plain text
- Retrieve differences between two blocks of text
- Create a set of patches for converting a block of text into another
- Apply a set of patches onto a block of text to convert it to another block of text
- Uses best-effort to apply patch even when the underlying text doesn't fully match.

#### Original

This code is forked originally from [diff_match_patch.rs](https://crates.io/crates/diff_match_patch), licensed under the [MIT](https://choosealicense.com/licenses/mit/) license.
