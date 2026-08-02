# RAG Indexing Pipeline in Rust

A command-line pipeline that reads text documents, creates embeddings with [Candle](https://github.com/huggingface/candle), and stores them in [LanceDB](https://lancedb.com/). It accompanies the Towards Data Science article, [*Scale Up Your RAG: A Rust-Powered Indexing Pipeline with LanceDB and Candle*](https://medium.com/towards-data-science/scale-up-your-rag-a-rust-powered-indexing-pipeline-with-lancedb-and-candle-cc681c6162e8).

## What it does

- Chunks plain-text documents and processes input files in parallel.
- Generates `all-MiniLM-L6-v2` embeddings with Candle.
- Persists vectors and source filenames in LanceDB.

## Prerequisites

- Rust 1.91 or newer (install the current stable [Rust toolchain](https://www.rust-lang.org/tools/install))
- Network access on the first run, so the embedding model can be downloaded from Hugging Face

## Quick start

```bash
git clone https://github.com/mkaydin/rag_indexing_rust.git
cd rag_indexing_rust
cargo run --release -- --input-directory tests/fixtures/documents --db-uri data/vecdb1
```

The database is created under `data/`, which is intentionally ignored by Git. Bring your own directory of `.txt` files by replacing `--input-directory`.

## Repository layout

```text
src/                    Rust application source
tests/fixtures/         Small text fixtures used by the end-to-end test
scripts/                Utility scripts, including large-fixture generation
docs/notebooks/         Exploratory Jupyter notebooks
data/                   Local LanceDB output (generated, ignored)
```

## Development

```bash
cargo fmt --check
cargo check --all-targets
cargo test
```

Some tests download and load the embedding model, so their first execution may take longer and requires network access.
