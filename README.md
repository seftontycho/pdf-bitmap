# pdf-bitmap

Extracts all jpgs out of a pdf.

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [Maturin](https://github.com/PyO3/maturin)

## Installation

Clone this repository and run the following command in the root directory:

```bash
maturin build
pip install target/wheels/pdf_bitmap-*.whl
```

## Usage

```python
from pdf_bitmap import pdf_2_jpgs

with open("example.pdf", "rb") as f:
    jpgs = pdf_2_jpgs(f.read())

for i, jpg in enumerate(jpgs):
    with open(f"output_{i}.jpg", "wb") as f:
        f.write(jpg)
```

