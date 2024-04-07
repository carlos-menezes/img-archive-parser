#[cfg(test)]
mod tests {
    use img_archive_parser::IMGArchive;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    #[test]
    fn it_should_load_a_valid_file() {
        let file = File::open("tests/archive.img").unwrap();
        let buf_reader = BufReader::new(file);
        let img_archive = IMGArchive::new(buf_reader);
        assert_eq!(img_archive.is_ok(), true);
    }

    #[test]
    fn it_should_not_load_an_invalid_file() {
        let fake_file: Vec<u8> = vec![0x56, 0x44, 0x52, 0x32, 0x7A, 0x02, 0x00, 0x00];
        let buf_reader = BufReader::new(Cursor::new(fake_file));
        let img_archive = IMGArchive::new(buf_reader);
        assert_eq!(img_archive.is_err(), true);
    }

    #[test]
    fn it_should_extract_an_entry() {
        let file = File::open("tests/archive.img").unwrap();
        let buf_reader = BufReader::new(file);
        let mut img_archive = IMGArchive::new(buf_reader).unwrap();
        let entry = img_archive.next().unwrap().unwrap();

        let mut writer = Cursor::new(Vec::new());
        let result = img_archive.extract(&entry, &mut writer);
        assert!(result.is_ok());

        // Check that the writer now contains the extracted data
        let extracted_data = writer.into_inner();
        assert_eq!(extracted_data.len(), entry.streaming_size as usize * 2048);
    }
}
