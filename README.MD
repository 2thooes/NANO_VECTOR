<div align="center">

# NANO_VECTOR

**A tiny file-backed vector database with an interactive shell, written in Rust.**


</div>

---

## Overview

NANO_VECTOR lets you create named vector databases, store labelled `f32` vectors in them, and run similarity queries, all from a small interactive command-line shell. Each database is one compact binary file stored in your operating system's standard application data directory. There is no server, no configuration, and no external service.

It is built for learning, prototyping, and small datasets where a full vector database would be overkill.

## Features

- **Interactive shell** with a `~[db_name]~` prompt, similar in spirit to `psql` or `sqlite3`
- **Multiple named databases**: create, list, enter, leave, and drop them
- **Fixed-width binary format** with O(1) random access to any vector by index
- **Four similarity and distance metrics**: cosine, dot product, Euclidean, Manhattan
- **k-nearest-neighbour search** (`closest`) relative to any stored vector
- **Pairwise distance** between any two stored vectors
- **Cross-platform storage** via the [`directories`](https://crates.io/crates/directories) crate

## Installation

Requires Rust with **edition 2024** support (Rust 1.85 or newer).

```bash
git clone https://github.com/2thooes/NANO_VECTOR.git
cd NANO_VECTOR
cargo run --release
```

Databases are stored in the platform data directory:

| OS      | Location                                                  |
|---------|-----------------------------------------------------------|
| Linux   | `~/.local/share/nano_vector/`                             |
| macOS   | `~/Library/Application Support/com.2thooes.nano_vector/`  |
| Windows | `%APPDATA%\2thooes\nano_vector\data\`                     |

Each database is saved as `<name>.bin`.

## Quick Start

```text
~~newdb animals 3          # create a 3-dimensional database and enter it
Success!
~[animals]~put cat 0.9 0.1 0.0
put done
~[animals]~put dog 0.8 0.3 0.1
put done
~[animals]~put car 0.0 0.2 0.9
put done
~[animals]~get all
cat <<0.9><0.1><0>>
dog <<0.8><0.3><0.1>>
car <<0><0.2><0.9>>
Success!
~[animals]~closest cosine 2 0     # 2 nearest neighbours of vector #0 (cat)
1:0.963
2:0.024
Success!
~[animals]~distance euclid 0 1
0.2449
~[animals]~exitdb
Success!
```

> Outputs are shown rounded. Vector ids are 0-based and follow insertion order.

## Command Reference

### Database management

| Command | Description |
|---------|-------------|
| `help` | Show help |
| `newdb <name> <dimensions> [label_length]` | Create a database and enter it. `label_length` is the fixed byte width reserved for each label (default `32`, max `255`) |
| `enterdb <name>` | Open an existing database |
| `exitdb` | Leave the current database |
| `dropdb [name]` | Delete a database by name, or the current one if no name is given |
| `lsdb` | List all databases |
| `status` | Show whether you are inside a database and how many vectors it holds |

### Working with vectors

These commands require you to be inside a database.

| Command | Description |
|---------|-------------|
| `put <label> <x1> … <xN>` | Append a labelled vector. It must have exactly `N = dimensions` components, and the label must fit in `label_length` bytes |
| `get all` | Print every stored vector |
| `get id <i>` | Print the vector with index `i` |
| `distance <metric> <i> <j>` | Compute the metric between vectors `i` and `j` |
| `closest <metric> <k> <i>` | Print the `k` vectors nearest to vector `i` as `id:score` lines, excluding `i` itself |

### Metrics

| Name | Meaning | `closest` ordering |
|------|---------|--------------------|
| `cosine` | Cosine similarity | Highest first |
| `dot` | Dot product | Highest first |
| `euclid` | Euclidean (L2) distance | Lowest first |
| `manhattan` | Manhattan (L1) distance | Lowest first |

## How It Works

### Storage format

A database is a single little-endian binary file with a 9-byte header followed by fixed-size records:

```text
┌───────────────────────── header (9 bytes) ─────────────────────────┐
│ dimensions : u64 (8 bytes) │ label_length : u8 (1 byte)            │
├────────────────────────────────────────────────────────────────────┤
│ record 0 │ label: [u8; label_length] (zero-padded) │ f32 × dims    │
│ record 1 │ label: [u8; label_length] (zero-padded) │ f32 × dims    │
│   ...                                                              │
└────────────────────────────────────────────────────────────────────┘

record_size = label_length + dimensions × 4
offset(i)   = 9 + i × record_size
```

Because every record has the same size, fetching vector `i` is a single seek, with no index structure required. The number of vectors is derived from the file size.

### Query execution

`closest` performs an **exact brute-force scan**. It reads every stored vector, scores it against the query vector using the chosen metric, sorts the results, and returns the top `k`. Results are exact, not approximate, and cost grows linearly with database size.

### Project layout

```text
src/
├── main.rs              # REPL loop and command dispatch
├── input.rs             # reading and tokenising user input
├── command.rs           # parsed command representation (body / main arg / sub args)
├── db.rs                # Db handle: path, open file, dimensions, label length
├── functions.rs         # top-level command implementations and validation
├── subfunctions.rs      # get / distance / closest implementations
└── helper_functions.rs  # file reads, vector loading, metric math
```

## Limitations

NANO_VECTOR is an early-stage (v0.1.0) project. Current constraints:

- Storage is **append-only**: there is no update or delete for individual vectors (only `dropdb` for a whole database)
- Search is **brute force** with no ANN index (HNSW, IVF, etc.), so it suits small to medium datasets
- Vectors are `f32` only
- Labels are limited to `label_length` bytes (at most 255) and are truncated at the first zero byte when displayed
- `closest` and `distance` accept only the index of an already stored vector, not an arbitrary query vector

## Roadmap

- [ ] Query by arbitrary vector, not only by stored index
- [ ] Update and delete operations
- [ ] Approximate nearest-neighbour index
- [ ] Unit and integration tests
- [ ] Reuse the engine as a library crate alongside the CLI

## Contributing

Issues and pull requests are welcome. Please run `cargo fmt` and `cargo clippy` before submitting.

## License

No license has been chosen yet. Add a `LICENSE` file (for example MIT or Apache-2.0) and update this section.

## Author

Created by **Tymur Mikhaliev** ([@2thooes](https://github.com/2thooes)).