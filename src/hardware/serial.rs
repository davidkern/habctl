pub struct Serial {
    serial: mio_serial::Serial,
}

impl From<mio_serial::Serial> for Serial {
    fn from(serial: mio_serial::Serial) -> Serial {
        Serial { serial }
    }
}