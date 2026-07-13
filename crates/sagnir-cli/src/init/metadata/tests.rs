use std::io::{self, Read};

use super::read_bounded_reader;

#[test]
fn bounded_reader_collects_short_reads_until_eof() -> io::Result<()> {
    let reader = ShortReader::new(b"format = 1\nprofile = \"standard\"\n", 3);
    let (buffer, len) = read_bounded_reader::<64, _>(reader)?;

    assert_eq!(&buffer[..len], b"format = 1\nprofile = \"standard\"\n");
    Ok(())
}

#[test]
fn bounded_reader_rejects_trailing_data_after_short_reads() {
    let reader = ShortReader::new(b"123456789", 2);
    let result = read_bounded_reader::<8, _>(reader);
    assert!(result.is_err());
    let Err(error) = result else { return };

    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("exceeds its size limit"));
}

struct ShortReader<'a> {
    content: &'a [u8],
    offset: usize,
    chunk: usize,
}

impl<'a> ShortReader<'a> {
    fn new(content: &'a [u8], chunk: usize) -> Self {
        Self {
            content,
            offset: 0,
            chunk,
        }
    }
}

impl Read for ShortReader<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let remaining = &self.content[self.offset..];
        let len = remaining.len().min(output.len()).min(self.chunk);
        output[..len].copy_from_slice(&remaining[..len]);
        self.offset += len;
        Ok(len)
    }
}
