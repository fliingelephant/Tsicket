
## 清易票 *Tsicket*

- Three modules:

  - ***[tsicket_app](https://github.com/fliingelephant/Tsicket/app)*** (binary): the main module
  - ***[tsicket_db](https://github.com/fliingelephant/Tsicket/db)*** (library): for database
  - ***[tsicket_utils](https://github.com/fliingelephant/Tsicket/utils)*** (library): for basic facilities such as error management, etc.

- Directory structure:

  ```shell
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
  ```
  

