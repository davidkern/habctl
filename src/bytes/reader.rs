// use std::io::BufReader;
// use std::io::BufRead;
// use std::io::Read;

// pub struct Reader<TSource, TItem>
// where TSource: Iterator<Item=TItem>
// {
//     source: TSource,
// }

// pub type ByteReader<TSource> = Reader<TSource, u8>;

// impl<TSource, TItem> Reader<TSource, TItem>
// where TSource: Iterator<Item=TItem>
// {
//     pub fn new(source: TSource) -> Self {
//         Self {
//             source
//         }
//     }

//     pub fn read_until(&mut self, pattern: &[TItem]) -> Option<Vec<TItem>> {
//         let buf = Vec::new();
//         let pattern_idx = 0;

//         loop {
//             match self.source.next() {
//                 Ok()
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_read_until() {
//         let lf = [0x0a];
//         let mut lf_slice: &[u8] = &lf;
//         let lf_reader = Reader::new(lf_slice.iter());
//     }
// }


// trait Reader: Iterator {
//     fn read_until(&mut self, pattern: &[Self::Item]) -> Option<Vec<Self::Item>>;
// }

// pub struct ByteReader<R>
// where R: Read
// {
//     source: BufReader<R>
// }

// impl ByteReader {
//     fn read_until_ends_with(&mut self, pattern: &[u8]) -> Option<Vec<u8>>;
// }

// impl<R> ByteReader for BufReader<R>
// where
//     R: Read
// {
//     fn read_until_ends_with(&mut self, pattern: &[u8]) -> Option<Vec<u8>> {
//         let mut buf = Vec::new();

//         loop {
//             match self.read_until(*pattern.last().unwrap(), &mut buf) {
//                 // End of stream or error returns None
//                 Ok(0) | Err(_) => { break None; },
    
//                 // Return buffer if it ends with pattern
//                 Ok(_) => if buf.ends_with(pattern) { break Some(buf); }
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::bytes::CRLF;

//     fn read_crlf_line<R>(reader: &mut R) -> Option<Vec<u8>>
//     where R: ByteReader
//     {
//         reader.read_until_ends_with(CRLF)
//     }

//     #[test]
//     fn test_read_crlf_line() {
//         // check empty => None
//         let empty = [].into();
//         let mut empty_slice: &[u8] = &empty;
//         assert!(read_crlf_line(&mut empty_slice) == None);

//     //     // check LF => None
//     //     let lf = [0x0a];
//     //     let mut lf_slice: &[u8] = &lf;
//     //     assert!(read_crlf_line(&mut lf_slice) == None);

//     //     // check CRLF => [Some(CRLF), None]
//     //     let crlf = [0x0d, 0x0a];
//     //     let mut crlf_slice: &[u8] = &crlf;
//     //     assert!(read_crlf_line(&mut crlf_slice) == Some(crlf.to_vec()));
//     //     assert!(read_crlf_line(&mut crlf_slice) == None);

//     //     // check LFCRLF => [Some(LFCRLF), None]
//     //     let lfcrlf = [0x0a, 0x0d, 0x0a];
//     //     let mut lfcrlf_slice: &[u8] = &lfcrlf;
//     //     assert!(read_crlf_line(&mut lfcrlf_slice) == Some(lfcrlf.to_vec()));
//     //     assert!(read_crlf_line(&mut lfcrlf_slice) == None);

//     //     // check [b'A', CFLF, b'B', CRLF] => [Some(b'A'), Some(b'B'), None]
//     //     let fields = [b'A', 0x0d, 0x0a, b'B', 0x0d, 0x0a];
//     //     let mut fields_slice: &[u8] = &fields;
//     //     assert!(read_crlf_line(&mut fields_slice) == Some(fields[0..3].to_vec()));
//     //     assert!(read_crlf_line(&mut fields_slice) == Some(fields[3..6].to_vec()));
//     //     assert!(read_crlf_line(&mut fields_slice) == None);
//     // }
// }
