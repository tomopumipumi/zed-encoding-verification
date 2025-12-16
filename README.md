# Zed Encoding Preservation Verification

This repository contains verification scripts and test data used to validate the encoding preservation fixes for Zed.

The primary goal is to ensure that files opened and saved using the patched version of Zed retain their exact byte sequence.

## 📂 Repository Structure

The target_files directory contains the test data:

- target_files/originals/: The original set of test files provided during the review process. This includes various encodings (ISO-8859 variants, UTF-16LE/BE without BOM, etc.).

- target_files/saved/: The result of opening the originals in the patched Zed build and executing a save operation (without modifying the content).

Plaintext

```
.
├── src/
│ └── main.rs # The verification script
├── target_files/
│ ├── originals/ # Source of truth (Reference)
│ └── saved/ # Files saved by the patched Zed
└── README.md
```

## 🚀 Usage

To run the verification, execute the following command in the root of the repository:

```Bash
cargo run
```

### What this script does

- Iterates through all files in target_files/originals/.

- Finds the corresponding file in target_files/saved/.

- Performs a strict byte-for-byte comparison.

- Reports PASS if the files are identical, or FAIL with a hex dump of the mismatch if they differ.

## ✅ Expected Output

If the encoding fixes are working correctly, the output should look like this:
Plaintext

```
📂 Target Directory: "target_files"

✅ PASS: Big5.txt
✅ PASS: EUC-JP.txt
...
✅ PASS: UTF-16_LE.txt
✅ PASS: UTF-16_BE.txt
...

---

Total: 32
Passed: 32
All files matched perfectly!
```

## 📝 Notes on Verification Logic

This verification ensures that:

- BOM Preservation: Files with a BOM retain it upon saving.

- BOM-less Heuristics: Files without a BOM (specifically UTF-16LE/BE) are correctly detected via the added heuristics and saved without adding an artificial BOM or converting to UTF-8.

- No Data Corruption: The content remains bit-exact after the load/save cycle.
