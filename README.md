# img-archive-parser

![ci workflow](https://github.com/carlos-menezes/img-archive-parser/actions/workflows/ci.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/img-archive-parser)

This library provides a Rust implementation for reading and extracting files from [IMG Archives](https://gtamods.com/wiki/IMG_archive).

## Features

- Read IMG archives
- Extract files from IMG archives
- Iterate over entries in an IMG archive

## Usage

To read an IMG archive, create a new `IMGArchive` instance:

```rust
let file = File::open("path_to_your_img_file.img")?;
let buf_reader = BufReader::new(file);
let mut img_archive = IMGArchive::new(buf_reader)?;
```

To extract a file from the IMG archive, use the `extract` method:

```rust
let entry = img_archive.next().unwrap()?;
let mut output = File::create("output_file")?;
img_archive.extract(&entry, &mut output)?;
```

## Structures

- `Entry`: Represents an entry in the IMG archive. It contains the offset, streaming size, archive size, and file name of the entry.
- `IMGArchive`: Represents an IMG archive. It provides methods for reading the archive and extracting files.

## License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
