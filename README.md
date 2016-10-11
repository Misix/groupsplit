# Group Split

This program splits a delimeted file into multiple files, based on the value of the specified column. Rows that share the same value are concatenated into the same file.

# Building

Build this like any other Rust program.

```bash
cd groupsplit
cargo build --release
```

The resulting binaries are available in `target/release`.

# Usage

Please pipe all input over stdin.

```bash
cat file_to_split.tsv | groupsplit
```

# Maintainer

Davis Remmel <dremmel@misix.com>

# License

Group Split
Copyright (C) 2016  Misix, Inc.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.
