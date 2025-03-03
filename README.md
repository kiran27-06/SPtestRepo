# SPtestRepo

Collaborators:
Kiran & Manmath

### **Implementation Plan for Hashassin Project**

---

#### **Phase 1: Setup and Project Structure (Feb 10 - Feb 14)**
- [ ] Create a GitHub repository from the provided GitHub Classroom link.
- [ ] Setup project structure:
  - [ ] Create a Rust workspace with two crates:
    - `cli` (binary crate)
    - `core` (library crate)
- [ ] Ensure `core` crate can be accessed using `use hashassin_core::*`
- [ ] Initialize Cargo projects inside respective directories.
- [ ] Add basic README.md, CREDITS.md, and HONESTY.md.

---

#### **Phase 2: Implement `gen-passwords` Command (Feb 15 - Feb 20)**
- [ ] Implement random password generation:
  - [ ] Use ASCII characters (both cases, punctuation, and spaces).
  - [ ] Ensure no non-printable characters.
- [ ] Implement CLI options:
  - [ ] `--chars` (default: 4, 8-bit wide)
  - [ ] `--out-file` (write to a file if specified, else stdout)
  - [ ] `--threads` (multi-threading for password generation)
  - [ ] `--num` (specify number of passwords)
- [ ] Ensure proper error handling for invalid input values.
- [ ] Write unit tests and integration tests for this functionality.

---

#### **Phase 3: Implement `gen-hashes` Command (Feb 21 - Feb 26)**
- [ ] Implement hash generation from input passwords.
- [ ] Implement CLI options:
  - [ ] `--in-file` (read passwords from file)
  - [ ] `--out-file` (write output in specified format)
  - [ ] `--threads` (multi-threaded hashing)
  - [ ] `--algorithm` (choose hashing algorithm)
- [ ] Support hashing algorithms:
  - [ ] MD5
  - [ ] SHA-256
  - [ ] SHA3-512
  - [ ] Scrypt
- [ ] Implement correct output file format:
  - [ ] Version byte
  - [ ] Algorithm length byte
  - [ ] Algorithm name
  - [ ] Password length byte
  - [ ] Hashed passwords with zero padding
- [ ] Ensure proper error handling and validation.
- [ ] Write tests for hashing functionality.

---

#### **Phase 4: Implement `dump-hashes` Command (Feb 27 - Mar 3)**
- [ ] Implement functionality to read and parse hash files.
- [ ] Implement CLI option:
  - [ ] `--in-file` (read generated hash file)
- [ ] Implement correct output format:
  - [ ] Version number
  - [ ] Algorithm used
  - [ ] Password length
  - [ ] List of decoded hashes
- [ ] Ensure proper error handling.
- [ ] Write unit and integration tests.

---

#### **Phase 5: Code Quality & Optimization (Mar 4 - Mar 10)**
- [ ] Run `cargo fmt` and ensure formatting compliance.
- [ ] Run `cargo clippy` and fix warnings.
- [ ] Implement logging using `tracing` crate.
- [ ] Ensure no `unwrap()` or `expect()` calls.
- [ ] Implement structured error handling.
- [ ] Add support for additional hashing algorithms (optional).
- [ ] Optimize threading performance.
- [ ] Write proper documentation (`cargo doc --document-private-items --no-deps`).

---

#### **Phase 6: Final Testing & Submission (Mar 11 - Mar 18)**
- [ ] Conduct thorough testing.
- [ ] Ensure CLI commands work as expected.
- [ ] Prepare README.md with:
  - [ ] Instructions to run the program.
  - [ ] Libraries used.
  - [ ] Additional features implemented.
- [ ] Submit the project before **March 24th, 11:59 PM**.

---

