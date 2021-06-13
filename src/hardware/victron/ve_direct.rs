//! Victron VE-Direct interface
use anyhow::Result;
use bytes::{Buf, BytesMut};
use std::path::Path;
use serial_io::{build, AsyncSerial};
use std::time::Duration;
use tokio_util::codec::{Decoder, FramedRead};
use std::str;
use combine::{
    error::{ParseError, StreamError},
    parser::{
        byte::digit,
        combinator::{any_partial_state, AnyPartialState},
        range::{range, recognize, take},
    },
    skip_many, skip_many1,
    stream::{easy, PartialStream, RangeStream, StreamErrorFor},
    Parser,
};
use std::num::Wrapping;
use tokio_stream::StreamExt;
use std::cell::Cell;

#[derive(Default)]
pub struct VeDirectMppt {
    name: String,
    path: String,
    pub telemetry: Cell<MpptFrame>,
}

impl VeDirectMppt {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_owned(),
            path: path.to_owned(),
            telemetry: Cell::default(),
        }
    }

    /// Process data from a VeDirect device
    pub async fn run(&self) -> Result<()> {
        let builder = build(self.path.as_str(), 19200);
        let serial = AsyncSerial::from_builder(&builder)?;

        let decoder = VeDirectMpptDecoder::default();
        let mut frame_reader = FramedRead::new(serial, decoder);

        while let Some(result) = frame_reader.next().await {
            match result {
                Ok(frame) => {
                    println!("{}", frame);
                    self.telemetry.set(frame);
                },
                Err(e) => { println!("error: {}", e); },
            }
        }
        
        Ok(())
    }
}

pub struct VeDirectMpptDecoder {
    state: State,
}

impl Default for VeDirectMpptDecoder {
    fn default() -> Self {
        Self { state: State::Unsynchronized }
    }
}

struct Cursor<'a> {
    point: usize,
    bytes: &'a mut BytesMut,
    checksum: Wrapping<u8>,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a mut BytesMut) -> Self {
        Self { point: 0, bytes, checksum: Wrapping(0) }
    }

    fn byte(&mut self) -> Option<&u8> {
        let point = self.point;
        self.point += 1;
        if let Some(byte) = self.bytes.get(point) {
            self.checksum += Wrapping(*byte);
            Some(byte)
        } else {
            None
        }
    }

    fn read_until(&mut self, pattern: &[u8]) -> Option<Vec<u8>> {
        let mut output = Vec::new();
        let mut idx = 0;
        let len = pattern.len();
        let mut checksum = Wrapping(0u8);

        if len == 0 { return None }

        loop {
            if idx == len {
                // success
                self.checksum += checksum;
                break Some(output)
            }

            if let Some(byte) = self.bytes.get(self.point + idx) {

                if Some(byte) == pattern.get(idx) {
                    // matching, advance the index
                    idx += 1;
                    checksum += Wrapping(*byte);
                } else {
                    // failed, move point and reset index
                    output.push(*byte);
                    self.point += 1;
                    idx = 0;
                    checksum = Wrapping(0u8);
                }
            } else {
                // out of input
                break None
            }
        }
    }

    fn consume_to_point(&mut self) {
        self.bytes.advance(self.point);
        self.point = 0;
    }

    fn clear_checksum(&mut self) {
        self.checksum = Wrapping(0u8);
    }

    fn is_checksum_valid(&mut self) -> bool {
        self.checksum.0 == 0
    }
}

enum State {
    Unsynchronized,
    Crlf,
    Name,
    Tab,
    Value,
}

#[derive(Default, Debug)]
pub struct MpptFrame {
    /// V: Battery voltage (mV)
    battery_voltage: Option<u32>,

    /// VPV: Panel voltage (mV)
    panel_voltage: Option<u32>,

    /// PPV: Panel power (W)
    panel_power: Option<u32>,

    /// I: Battery current (mA): >0 charging, <0 discharging
    battery_current: Option<i32>,

    /// IL: Load current (mA)
    load_current: Option<i32>,

    /// LOAD: Load status
    load_state: Option<bool>,

    /// RELAY: Relay state
    relay_state: Option<bool>,

    /// OR: Off reason
    off_reason: Option<OffReason>,

    /// H19: Yield total, resettable (10W)
    yield_total: Option<u32>,

    /// H20: Yield today (10W)
    yield_today: Option<u16>,

    /// H21: Maximum power today (W)
    maximum_power_today: Option<u16>,

    /// H22: Yield yesterday (10W)
    yield_yesterday: Option<u16>,

    /// H23: Maximum power yesterday (W)
    maximum_power_yesterday: Option<u16>,

    /// ERR: Error code
    error: Option<ErrorCode>,

    /// CS: Operating status
    state: Option<StateOfOperation>,

    /// FW: Firmware version. Whole number, potentially prefixed by a letter
    firmware_version: Option<String>,

    /// PID: Product Id
    product_id: Option<u32>,

    /// SER#: Serial number
    /// LLYYMMSSSSS - LL location, YYWW production data, SSSSS unique id
    serial_number: Option<String>,

    /// HSDS: Historical day sequence number 0..364
    day_number: Option<u16>,

    /// MPPT: Mppt Status
    mppt_status: Option<Mppt>,
}

impl std::fmt::Display for MpptFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VPV {:?} PPV {:?} V {:?} I {:?} H20 {:?} H21 {:?} CS {:?} MPPT {:?}",
            self.panel_voltage,
            self.panel_power,
            self.battery_voltage,
            self.battery_current,
            self.yield_today,
            self.maximum_power_today,
            self.state,
            self.mppt_status,
        )
    }    
}

impl Decoder for VeDirectMpptDecoder {
    type Item = MpptFrame;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(src);
        let mut name = String::new(); 
        let mut frame = MpptFrame::default();

        loop {
            match self.state {
                State::Unsynchronized => {
                    if cursor.read_until(b"\r\n").is_none() {
                        cursor.consume_to_point();
                        return Ok(None)
                    };
    
                    cursor.clear_checksum();

                    self.state = State::Crlf;
                },
    
                State::Crlf => {
                    if cursor.byte() != Some(&0x0d) {
                        self.state = State::Unsynchronized;
                        continue;
                    }

                    if cursor.byte() != Some(&0x0a) {
                        self.state = State::Unsynchronized;
                        continue;
                    }

                    self.state = State::Name;
                },

                State::Name => {
                    if let Some(name_bytes) = cursor.read_until(b"\t") {
                        match std::str::from_utf8(&name_bytes) {
                            Ok(n) => {
                                name = n.to_string();
                                self.state = State::Tab;
                            },
                            Err(_) => {
                                self.state = State::Unsynchronized;
                            }
                        }
                        continue;
                    } else {
                        break Ok(None)
                    }
                },

                State::Tab => {
                    if let Some(tab) = cursor.byte() {
                        self.state = State::Value;
                        continue;
                    } else {
                        break Ok(None)
                    }
                },

                State::Value => {
                    if let Some(value) = cursor.read_until(b"\r\n") {
                        match name.as_str() {
                            "V" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        frame.battery_voltage = Some(v);
                                    }    
                                }
                            },
                            "VPV" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        frame.panel_voltage = Some(v);
                                    }    
                                }
                            },
                            "PPV" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        frame.panel_power = Some(v);
                                    }
                                }
                            },
                            "I" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = i32::from_str_radix(&value_str, 10) {
                                        frame.battery_current = Some(v);
                                    }
                                }
                            },
                            "IL" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = i32::from_str_radix(&value_str, 10) {
                                        frame.load_current = Some(v);
                                    }
                                }  
                            },
                            "LOAD" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if value_str == "ON" {
                                        frame.load_state = Some(true);
                                    } else if value_str == "OFF" {
                                        frame.load_state = Some(false);
                                    }
                                }
                            },
                            "RELAY" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if value_str == "ON" {
                                        frame.relay_state = Some(true);
                                    } else if value_str == "OFF" {
                                        frame.relay_state = Some(false);
                                    }
                                }
                            },
                            "OR" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str[2..], 16) {
                                        if let Some(or) = OffReason::from_bits(v) {
                                            frame.off_reason = Some(or);
                                        }
                                    }
                                }  
                            },
                            "H19" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        frame.yield_total = Some(v);
                                    }
                                }                                  
                            },
                            "H20" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u16::from_str_radix(&value_str, 10) {
                                        frame.yield_today = Some(v);
                                    }
                                }  
                            },
                            "H21" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u16::from_str_radix(&value_str, 10) {
                                        frame.maximum_power_today = Some(v);
                                    }
                                }  
                            },
                            "H22" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u16::from_str_radix(&value_str, 10) {
                                        frame.yield_yesterday = Some(v);
                                    }
                                }  
                            },
                            "H23" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u16::from_str_radix(&value_str, 10) {
                                        frame.maximum_power_yesterday = Some(v);
                                    }
                                }  
                            },
                            "ERR" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        if let Some(err) = ErrorCode::from_u32(v) {
                                            frame.error = Some(err);
                                        }
                                    }
                                }
                            },
                            "CS" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        if let Some(cs) = StateOfOperation::from_u32(v) {
                                            frame.state = Some(cs);
                                        }
                                    }
                                }
                            },
                            "FW" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    frame.firmware_version = Some(String::from(value_str));
                                }
                            },
                            "PID" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str[2..], 16) {
                                        frame.product_id = Some(v);
                                    }
                                }
                            },
                            "SER#" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    frame.serial_number = Some(String::from(value_str));
                                }
                            },
                            "HSDS" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u16::from_str_radix(&value_str, 10) {
                                        frame.day_number = Some(v);
                                    }
                                }
                            },
                            "MPPT" => {
                                if let Ok(value_str) = str::from_utf8(&value) {
                                    if let Ok(v) = u32::from_str_radix(&value_str, 10) {
                                        if let Some(mppt) = Mppt::from_u32(v) {
                                            frame.mppt_status = Some(mppt);
                                        }
                                    }
                                }
                            },
                            "Checksum" => {
                                if cursor.is_checksum_valid() {
                                    self.state = State::Crlf;
                                    cursor.consume_to_point();
                                    break Ok(Some(frame))
                                } else {
                                    self.state = State::Unsynchronized;
                                    continue;
                                }
                            }
                            _ => {
                                self.state = State::Unsynchronized;
                                continue;
                            },
                        }
                        self.state = State::Crlf;
                        continue;
                    } else {
                        break Ok(None)
                    }
                }
            }    
        }
    }
}

bitflags! {
    pub struct OffReason: u32 {
        const NONE = 0x0000_0000;
        const NO_INPUT_POWER = 0x0000_0001;
        const SWITCHED_OFF_POWER_SWITCH = 0x0000_0002;
        const SWITCHED_OFF_REGISTER = 0x0000_0004;
        const REMOTE_INPUT = 0x0000_0008;
        const PROTECTION_ACTIVE = 0x0000_0010;
        const PAYGO = 0x0000_0020;
        const BMS = 0x0000_0040;
        const ENGINE_SHUTDOWN_DETECTION = 0x0000_0080;
        const ANALYSING_INPUT_VOLTAGE = 0x0000_0100;
    }
}

#[derive(Debug)]
pub enum StateOfOperation {
    Off,
    LowPower,
    Fault,
    Bulk,
    Absorption,
    Float,
    Storage,
    Equalize,
    Inverting,
    PowerSupply,
    StartingUp,
    RepeatedAbsorption,
    AutoEqualize,
    BatterySafe,
    ExternalControl,
}

impl StateOfOperation {
    fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(StateOfOperation::Off),
            1 => Some(StateOfOperation::LowPower),
            2 => Some(StateOfOperation::Fault),
            3 => Some(StateOfOperation::Bulk),
            4 => Some(StateOfOperation::Absorption),
            5 => Some(StateOfOperation::Float),
            6 => Some(StateOfOperation::Storage),
            7 => Some(StateOfOperation::Equalize),
            9 => Some(StateOfOperation::Inverting),
            11 => Some(StateOfOperation::PowerSupply),
            245 => Some(StateOfOperation::StartingUp),
            246 => Some(StateOfOperation::RepeatedAbsorption),
            247 => Some(StateOfOperation::AutoEqualize),
            248 => Some(StateOfOperation::BatterySafe),
            252 => Some(StateOfOperation::ExternalControl),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    NoError,
    BatteryVoltageHigh,
    ChargerTemperatureHigh,
    ChargerCurrentHigh,
    ChargerCurrentReversed,
    BulkTimeLimit,
    CurrentSensor,
    TerminalTemperatureHigh,
    Converter,
    InputVoltageHigh,
    InputCurrentHigh,
    InputShutdownDueToBatteryVoltage,
    InputShutdownDueToCurrentFlowWhileOff,
    LostCommunication,
    SynchronizedChargingConfiguration,
    BmsConnectionLost,
    NetworkMisconfigured,
    FactoryCalibrationDataLost,
    InvalidFirmware,
    InvalidUserSettings,
}

impl ErrorCode {
    fn from_u32(val: u32) -> Option<ErrorCode> {
        match val {
            0 => Some(ErrorCode::NoError),
            2 => Some(ErrorCode::BatteryVoltageHigh),
            17 => Some(ErrorCode::ChargerTemperatureHigh),
            18 => Some(ErrorCode::ChargerCurrentHigh),
            19 => Some(ErrorCode::ChargerCurrentReversed),
            20 => Some(ErrorCode::BulkTimeLimit),
            21 => Some(ErrorCode::CurrentSensor),
            26 => Some(ErrorCode::TerminalTemperatureHigh),
            28 => Some(ErrorCode::Converter),
            33 => Some(ErrorCode::InputVoltageHigh),
            34 => Some(ErrorCode::InputCurrentHigh),
            38 => Some(ErrorCode::InputShutdownDueToBatteryVoltage),
            39 => Some(ErrorCode::InputShutdownDueToCurrentFlowWhileOff),
            65 => Some(ErrorCode::LostCommunication),
            66 => Some(ErrorCode::SynchronizedChargingConfiguration),
            67 => Some(ErrorCode::BmsConnectionLost),
            68 => Some(ErrorCode::NetworkMisconfigured),
            116 => Some(ErrorCode::FactoryCalibrationDataLost),
            117 => Some(ErrorCode::InvalidFirmware),
            119 => Some(ErrorCode::InvalidUserSettings),
            _ => None,                    
        }
    }
}

#[derive(Debug)]
pub enum Mppt {
    Off = 0,
    VoltageOrCurrentLimited = 1,
    MpptTrackerActive = 2,
}

impl Mppt {
    fn from_u32(val: u32) -> Option<Mppt> {
        match val {
            0 => Some(Mppt::Off),
            1 => Some(Mppt::VoltageOrCurrentLimited),
            2 => Some(Mppt::MpptTrackerActive),
            _ => None,
        }        
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use tokio_util::codec::FramedRead;
    use futures::TryStreamExt;
    use super::{VeDirectMpptDecoder, MpptFrame};

    #[tokio::test]
    async fn parse() {
        // 0x0d, 0x0a
        // field-label
        // 0x09
        // value
        let input = std::include_bytes!("../../../test/usb-VictronEnergy_BV_VE_Direct_cable_VE46V0KW-if00-port0");

        let reader = &mut Cursor::new(input);
        let decoder = VeDirectMpptDecoder::default();

        //let mut frame_reader = FramedRead::new(reader, decoder);

        let result = FramedRead::new(reader, decoder).try_collect().await;
        let frames: Vec<MpptFrame> = result.unwrap();

        // TODO: Fix
        // should be 299, but the decoder doesn't handle checksum not immediately followed by \r\n
        assert_eq!(291, frames.len());
    }
}
