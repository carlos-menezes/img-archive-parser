use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom, Write};

const IMG_HEADER: u32 = 0x32524556;
const ENTRY_SIZE: u64 = 0x20;
const SECTOR_SIZE: u32 = 0x800;

#[repr(C)]
#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub streaming_size: u16,
    pub archive_size: u16,
    pub file_name: [u8; 24], // This should be parsed as a null-terminated string
}

#[derive(Debug)]
pub struct IMGArchive<B: Read + Seek> {
    buffer: B,
    entry_count: u32,
}

impl<B: Read + Seek> IMGArchive<B> {
    pub fn new(mut buffer: B) -> Result<Self, Box<dyn std::error::Error>> {
        let header = buffer.read_u32::<LittleEndian>()?;
        if header != IMG_HEADER {
            return Err("invalid .img file".into());
        }
        let entry_count = buffer.read_u32::<LittleEndian>()?;
        Ok(IMGArchive {
            buffer,
            entry_count,
        })
    }

    pub fn extract<W: Write>(&mut self, entry: &Entry, writer: &mut W) -> std::io::Result<()> {
        let current_pos = self.buffer.seek(SeekFrom::Current(0))?;

        // Every file contained in the img archive must be sector aligned,
        // where the size of each sector is 2048 bytes.
        // Thus values for offset and size have to be multiplied by 2048.
        let offset = (entry.offset as u64) * SECTOR_SIZE as u64;
        let size = (entry.streaming_size as u32) * SECTOR_SIZE;

        self.buffer.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; size as usize];
        self.buffer.read_exact(&mut buffer)?;
        writer.write_all(&buffer)?;

        // Seek back to the stored position
        self.buffer.seek(SeekFrom::Start(current_pos))?;

        Ok(())
    }
}

impl<B: Read + Seek> Iterator for IMGArchive<B> {
    type Item = Result<Entry, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_pos = match self.buffer.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos,
            Err(e) => return Some(Err(e.into())),
        };
        if (current_pos / ENTRY_SIZE) > self.entry_count as u64 {
            return None;
        }

        let offset = match self.buffer.read_u32::<LittleEndian>() {
            Ok(offset) => offset,
            Err(e) => return Some(Err(e.into())),
        };
        let streaming_size = match self.buffer.read_u16::<LittleEndian>() {
            Ok(size) => size,
            Err(e) => return Some(Err(e.into())),
        };
        let archive_size = match self.buffer.read_u16::<LittleEndian>() {
            Ok(size) => size,
            Err(e) => return Some(Err(e.into())),
        };

        let mut file_name = [0u8; 24];
        match self.buffer.read_exact(&mut file_name) {
            Ok(_) => (),
            Err(e) => return Some(Err(e.into())),
        };

        Some(Ok(Entry {
            offset,
            streaming_size,
            archive_size,
            file_name,
        }))
    }
}
