
// use std::io::BufReader;
// use crate::bytes::{
//     CRLF,
//     reader::ByteReader};

// pub static MPPT_DATA_1: &'static [u8] = include_bytes!("../../test/usb-VictronEnergy_BV_VE_Direct_cable_VE46V0KW-if00-port0");

// /// VE Direct data format supports text and hex frames
// #[derive(Debug)]
// pub enum VEDirectFrame {
//     Text(Vec<u8>),
//     Hex(Vec<u8>)
// }

// /// Iterator generating VE Direct Frames from a byte Reader source
// pub struct VEDirectFrameReader<Reader>
// where
//     Reader: ByteReader
// {
//     source: Reader,
// }

// impl<Reader> VEDirectFrameReader<Reader>
// where
//     Reader: ByteReader
// {
//     pub fn new(source: Reader) -> Self {
//         Self {
//             source
//         }
//     }
// }

// impl<Reader> Iterator for VEDirectFrameReader<Reader>
// where
//     Reader: ByteReader
// {
//     type Item = VEDirectFrame;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.source.read_until_ends_with(CRLF) {
//             None => None,
//             Some(buf) => Some(Self::Item::Text(buf))
//         }
//     }
// }

// pub fn run() {
//     let reader = VEDirectFrameReader::new(BufReader::new(MPPT_DATA_1));
//     for frame in reader {
//         println!("{:?}", frame);
//     }
// }

// // #[cfg(test)]
// // mod test {
// //     use super::*;

// // }

pub fn run() {}