### 清易票 *Tsicket*

- Three modules:

  - ***tsicket_app*** (binary)
  - ***tsicket_db*** (library)
  - ***tsicket_utils*** (library)

- Directory structure:

  .
  ├── Cargo.lock
  ├── Cargo.toml
  ├── README.md
  ├── app
  │  ├── Cargo.toml
  │  ├── README.md
  │  ├── src
  │  │  └── main.rs
  │  └── tsicket_app.iml
  ├── db
  │  ├── Cargo.toml
  │  ├── README.md
  │  ├── src
  │  │  └── lib.rs
  │  └── tsicket_db.iml
  ├── doc
  └── utils
    ├── Cargo.toml
    ├── README.md
    ├── src
    │  ├── error.rs
    │  ├── lib.rs
    │  └── logger.rs
    └── tsicket_utils.iml