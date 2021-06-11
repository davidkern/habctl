//! Victron VE-Direct interface

pub trait Description {
    fn description(&self) -> &str;
}

pub enum Measurement {
    /// Channel 1 voltage (mV)
    V(u32),

    /// Channel 2 voltage (mV)
    V2(u32),

    /// Channel 3 voltage (mV)
    V3(u32),

    /// Auxillary voltage (mV)
    Vs(u32),

    /// Mid-point voltage (mV)
    Vm(u32),

    /// Mid-point deviation (%)
    Dm(u8),

    /// Panel voltage (mV)
    Vpv(u32),

    /// Panel power (W)
    Ppv(u32),

    /// Channel 1 current (mA): >0 charging, <0 discharging
    I(i32),

    /// Channel 2 current (mA)
    I2(i32),

    /// Channel 3 current (mA)
    I3(i32),

    /// Load current (mA)
    Il(i32),

    /// Load status
    Load(bool),

    /// Battery temperature (deg C)
    T(i16),

    /// Instantaneous power (W)
    P(u32),

    /// Consumed mAhr
    Ce(u32),

    /// State of charge (%)
    Soc(u8),

    /// Time to go (minutes)
    Ttg(u16),

    /// Alarm state
    Alarm(bool),

    /// Relay state
    Relay(bool),

    /// Alarm reason
    Ar(AlarmReason),

    /// Off reason
    Or(OffReason),

    /// Depth of deepest discharge (mAhr)
    H1(u32),

    /// Depth of last discharge (mAhr)
    H2(u32),

    /// Depth of average discharge (mAhr)
    H3(u32),

    /// Charge cycle count
    H4(u32),

    /// Discharge count
    H5(u32),

    /// Cummulative draw (mAhr)
    H6(u32),

    /// Minimum main battery voltage (mV)
    H7(u16),

    /// Maximum main battery voltage (mV)
    H8(u16),

    /// Seconds since last full charge
    H9(u32),

    /// Automatic sync count
    H10(u32),

    /// Low main voltage alarm count
    H11(u32),

    /// High main voltage alarm count
    H12(u32),

    /// Low aux voltage alarm count
    H13(u32),

    /// High aux voltage alarm count
    H14(u32),

    /// Minimum aux voltage (mV)
    H15(u16),

    /// Maximum aux voltage (mV)
    H16(u16),

    /// Amount discharged (10W)
    H17(u32),

    /// Amount charged (10W)
    H18(u32),

    /// Yield total, resettable (10W)
    H19(u32),

    /// Yield today (10W)
    H20(u32),

    /// Maximum power today (W)
    H21(u16),

    /// Yield yesterday (10W)
    H22(u32),

    /// Maximum power yesterday (W)
    H23(u16),

    /// Error code
    Err(ErrorCode),

    /// Operating status
    Cs(StateOfOperation),

    /// Deprecated model description
    Bmv,

    /// Firmware version. Whole number, potentially prefixed by a letter
    Fw([u8; 4]),

    /// Firmware version. up to 6 digits, optional left 0 padding
    Fwe([u8; 6]),

    /// Product Id
    Pid(Pid),

    /// Serial number
    /// LLYYMMSSSSS - LL location, YYWW production data, SSSSS unique id
    Ser([u8; 11]),

    /// Historical day sequence number 0..364
    Hsds(u16),

    /// Device Mode
    Mode(Mode),

    /// AC output voltage (0.01V)
    AcOutV(u32),

    /// AC output current (0.1A)
    AcOutI(u32),

    /// AC output apparent power (VA)
    AcOutS(u32),

    /// Warning reason
    Warn(AlarmReason),

    /// Mppt Status
    Mppt(Mppt),
}

impl Description for Measurement {
    fn description(&self) -> &str {
        match self {
            Self::V(_) => "Main or channel 1 (battery) voltage",
            Self::V2(_) => "Channel 2 (battery) voltage",
            Self::V3(_) => "Channel 3 (battery) voltage",
            Self::Vs(_) => "Auxiliary (starter) voltage",
            Self::Vm(_) => "Mid-point voltage of the battery bank",
            Self::Dm(_) => "Mid-point deviation of the battery bank",
            Self::Vpv(_) => "Panel voltage",
            Self::Ppv(_) => "Panel power",
            Self::I(_) => "Main or channel 1 battery current",
            Self::I2(_) => "Channel 2 battery current",
            Self::I3(_) => "Channel 3 battery current",
            Self::Il(_) => "Load current",
            Self::Load(_) => "Load output state (ON/OFF)",
            Self::T(_) => "Battery temperature",
            Self::P(_) => "Instantaneous power",
            Self::Ce(_) => "Consumed Amp Hours",
            Self::Soc(_) => "State-of-charge",
            Self::Ttg(_) => "Time-to-go",
            Self::Alarm(_) => "Alarm condition active",
            Self::Relay(_) => "Relay state",
            Self::Ar(_) => "Alarm reason",
            Self::Or(_) => "Off reason",
            Self::H1(_) => "Depth of the deepest discharge",
            Self::H2(_) => "Depth of the last discharge",
            Self::H3(_) => "Depth of the average discharge",
            Self::H4(_) => "Number of charge cycles",
            Self::H5(_) => "Number of full discharges",
            Self::H6(_) => "Cumulative Amp Hours drawn",
            Self::H7(_) => "Minimum main (battery) voltage",
            Self::H8(_) => "Maximum main (battery) voltage",
            Self::H9(_) => "Number of seconds since last full charge",
            Self::H10(_) => "Number of automatic synchronizations",
            Self::H11(_) => "Number of low main voltage alarms",
            Self::H12(_) => "Number of high main voltage alarms",
            Self::H13(_) => "Number of low auxiliary voltage alarms",
            Self::H14(_) => "Number of high auxiliary voltage alarms",
            Self::H15(_) => "Minimum auxiliary (battery) voltage",
            Self::H16(_) => "Maximum auxiliary (battery) voltage",
            Self::H17(_) => "Amount of discharged energy",
            Self::H18(_) => "Amount of charged energy",
            Self::H19(_) => "Yield total (user resettable counter)",
            Self::H20(_) => "Yield today",
            Self::H21(_) => "Maximum power today",
            Self::H22(_) => "Yield yesterday",
            Self::H23(_) => "Maximum power yesterday",
            Self::Err(_) => "Error code",
            Self::Cs(_) => "State of operation",
            Self::Bmv => "Model description (deprecated)",
            Self::Fw(_) => "Firmware version (16 bit)",
            Self::Fwe(_) => "Firmware version (24 bit)",
            Self::Pid(_) => "Product ID",
            Self::Ser(_) => "Serial number",
            Self::Hsds(_) => "Day sequence number (0..364)",
            Self::Mode(_) => "Device mode",
            Self::AcOutV(_) => "AC output voltage",
            Self::AcOutI(_) => "AC output current",
            Self::AcOutS(_) => "AC output apparent power",
            Self::Warn(_) => "Warning reason",
            Self::Mppt(_) => "Tracker operation mode",    
        }
    }
}

bitflags! {
    pub struct AlarmReason: u32 {
        const LOW_VOLTAGE = 1;
        const HIGH_VOLTAGE = 2;
        const LOW_SOC = 4;
        const LOW_STARTER_VOLTAGE = 8;
        const HIGH_STARTER_VOLTAGE = 16;
        const LOW_TEMPERATURE = 32;
        const HIGH_TEMPERATURE = 64;
        const MID_VOLTAGE = 128;
        const OVERLOAD = 256;
        const DC_RIPPLE = 512;
        const LOW_V_AC_OUT = 1024;
        const HIGH_V_AC_OUT = 2048;
        const SHORT_CIRCUIT = 4096;
        const BMS_LOCKOUT = 8192;
    }
}

bitflags! {
    pub struct OffReason: u32 {
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

pub struct Pid(u32);

impl Description for Pid {
    fn description(&self) -> &str {
        match self.0 {
            0x203 => "BMV-700",
            0x204 => "BMV-702",
            0x205 => "BMV-700H",
            0x0300 => "BlueSolar MPPT 70|15*",
            0xA040 => "BlueSolar MPPT 75|50*",
            0xA041 => "BlueSolar MPPT 150|35*",
            0xA042 => "BlueSolar MPPT 75|15",
            0xA043 => "BlueSolar MPPT 100|15",
            0xA044 => "BlueSolar MPPT 100|30*",
            0xA045 => "BlueSolar MPPT 100|50*",
            0xA046 => "BlueSolar MPPT 150|70",
            0xA047 => "BlueSolar MPPT 150|100",
            0xA049 => "BlueSolar MPPT 100|50 rev2",
            0xA04A => "BlueSolar MPPT 100|30 rev2",
            0xA04B => "BlueSolar MPPT 150|35 rev2",
            0xA04C => "BlueSolar MPPT 75|10",
            0xA04D => "BlueSolar MPPT 150|45",
            0xA04E => "BlueSolar MPPT 150|60",
            0xA04F => "BlueSolar MPPT 150|85",
            0xA050 => "SmartSolar MPPT 250|100",
            0xA051 => "SmartSolar MPPT 150|100*",
            0xA052 => "SmartSolar MPPT 150|85*",
            0xA053 => "SmartSolar MPPT 75|15",
            0xA054 => "SmartSolar MPPT 75|10",
            0xA055 => "SmartSolar MPPT 100|15",
            0xA056 => "SmartSolar MPPT 100|30",
            0xA057 => "SmartSolar MPPT 100|50",
            0xA058 => "SmartSolar MPPT 150|35",
            0xA059 => "SmartSolar MPPT 150|100 rev2",
            0xA05A => "SmartSolar MPPT 150|85 rev2",
            0xA05B => "SmartSolar MPPT 250|70",
            0xA05C => "SmartSolar MPPT 250|85",
            0xA05D => "SmartSolar MPPT 250|60",
            0xA05E => "SmartSolar MPPT 250|45",
            0xA05F => "SmartSolar MPPT 100|20",
            0xA060 => "SmartSolar MPPT 100|20 48V",
            0xA061 => "SmartSolar MPPT 150|45",
            0xA062 => "SmartSolar MPPT 150|60",
            0xA063 => "SmartSolar MPPT 150|70",
            0xA064 => "SmartSolar MPPT 250|85 rev2",
            0xA065 => "SmartSolar MPPT 250|100 rev2",
            0xA066 => "BlueSolar MPPT 100|20",
            0xA067 => "BlueSolar MPPT 100|20 48V",
            0xA068 => "SmartSolar MPPT 250|60 rev2",
            0xA069 => "SmartSolar MPPT 250|70 rev2",
            0xA06A => "SmartSolar MPPT 150|45 rev2",
            0xA06B => "SmartSolar MPPT 150|60 rev2",
            0xA06C => "SmartSolar MPPT 150|70 rev2",
            0xA06D => "SmartSolar MPPT 150|85 rev3",
            0xA06E => "SmartSolar MPPT 150|100 rev3",
            0xA06F => "BlueSolar MPPT 150|45 rev2",
            0xA070 => "BlueSolar MPPT 150|60 rev2",
            0xA071 => "BlueSolar MPPT 150|70 rev2",
            0xA102 => "SmartSolar MPPT VE.Can 150/70",
            0xA103 => "SmartSolar MPPT VE.Can 150/45",
            0xA104 => "SmartSolar MPPT VE.Can 150/60",
            0xA105 => "SmartSolar MPPT VE.Can 150/85",
            0xA106 => "SmartSolar MPPT VE.Can 150/100",
            0xA107 => "SmartSolar MPPT VE.Can 250/45",
            0xA108 => "SmartSolar MPPT VE.Can 250/60",
            0xA109 => "SmartSolar MPPT VE.Can 250/70",
            0xA10A => "SmartSolar MPPT VE.Can 250/85",
            0xA10B => "SmartSolar MPPT VE.Can 250/100",
            0xA10C => "SmartSolar MPPT VE.Can 150/70 rev2",
            0xA10D => "SmartSolar MPPT VE.Can 150/85 rev2",
            0xA10E => "SmartSolar MPPT VE.Can 150/100 rev2",
            0xA10F => "BlueSolar MPPT VE.Can 150/100",
            0xA112 => "BlueSolar MPPT VE.Can 250/70",
            0xA113 => "BlueSolar MPPT VE.Can 250/100",
            0xA114 => "SmartSolar MPPT VE.Can 250/70 rev2",
            0xA115 => "SmartSolar MPPT VE.Can 250/100 rev2",
            0xA116 => "SmartSolar MPPT VE.Can 250/85 rev2",
            0xA201 => "Phoenix Inverter 12V 250VA 230V*",
            0xA202 => "Phoenix Inverter 24V 250VA 230V*",
            0xA204 => "Phoenix Inverter 48V 250VA 230V*",
            0xA211 => "Phoenix Inverter 12V 375VA 230V*",
            0xA212 => "Phoenix Inverter 24V 375VA 230V*",
            0xA214 => "Phoenix Inverter 48V 375VA 230V*",
            0xA221 => "Phoenix Inverter 12V 500VA 230V*",
            0xA222 => "Phoenix Inverter 24V 500VA 230V*",
            0xA224 => "Phoenix Inverter 48V 500VA 230V*",
            0xA231 => "Phoenix Inverter 12V 250VA 230V",
            0xA232 => "Phoenix Inverter 24V 250VA 230V",
            0xA234 => "Phoenix Inverter 48V 250VA 230V",
            0xA239 => "Phoenix Inverter 12V 250VA 120V",
            0xA23A => "Phoenix Inverter 24V 250VA 120V",
            0xA23C => "Phoenix Inverter 48V 250VA 120V",
            0xA241 => "Phoenix Inverter 12V 375VA 230V",
            0xA242 => "Phoenix Inverter 24V 375VA 230V",
            0xA244 => "Phoenix Inverter 48V 375VA 230V",
            0xA249 => "Phoenix Inverter 12V 375VA 120V",
            0xA24A => "Phoenix Inverter 24V 375VA 120V",
            0xA24C => "Phoenix Inverter 48V 375VA 120V",
            0xA251 => "Phoenix Inverter 12V 500VA 230V",
            0xA252 => "Phoenix Inverter 24V 500VA 230V",
            0xA254 => "Phoenix Inverter 48V 500VA 230V",
            0xA259 => "Phoenix Inverter 12V 500VA 120V",
            0xA25A => "Phoenix Inverter 24V 500VA 120V",
            0xA25C => "Phoenix Inverter 48V 500VA 120V",
            0xA261 => "Phoenix Inverter 12V 800VA 230V",
            0xA262 => "Phoenix Inverter 24V 800VA 230V",
            0xA264 => "Phoenix Inverter 48V 800VA 230V",
            0xA269 => "Phoenix Inverter 12V 800VA 120V",
            0xA26A => "Phoenix Inverter 24V 800VA 120V",
            0xA26C => "Phoenix Inverter 48V 800VA 120V",
            0xA271 => "Phoenix Inverter 12V 1200VA 230V",
            0xA272 => "Phoenix Inverter 24V 1200VA 230V",
            0xA274 => "Phoenix Inverter 48V 1200VA 230V",
            0xA279 => "Phoenix Inverter 12V 1200VA 120V",
            0xA27A => "Phoenix Inverter 24V 1200VA 120V",
            0xA27C => "Phoenix Inverter 48V 1200VA 120V",
            0xA281 => "Phoenix Inverter 12V 1600VA 230V",
            0xA282 => "Phoenix Inverter 24V 1600VA 230V",
            0xA284 => "Phoenix Inverter 48V 1600VA 230V",
            0xA291 => "Phoenix Inverter 12V 2000VA 230V",
            0xA292 => "Phoenix Inverter 24V 2000VA 230V",
            0xA294 => "Phoenix Inverter 48V 2000VA 230V",
            0xA2A1 => "Phoenix Inverter 12V 3000VA 230V",
            0xA2A2 => "Phoenix Inverter 24V 3000VA 230V",
            0xA2A4 => "Phoenix Inverter 48V 3000VA 230V",
            0xA340 => "Phoenix Smart IP43 Charger 12|50 (1+1)",
            0xA341 => "Phoenix Smart IP43 Charger 12|50 (3)",
            0xA342 => "Phoenix Smart IP43 Charger 24|25 (1+1)",
            0xA343 => "Phoenix Smart IP43 Charger 24|25 (3)",
            0xA344 => "Phoenix Smart IP43 Charger 12|30 (1+1)",
            0xA345 => "Phoenix Smart IP43 Charger 12|30 (3)",
            0xA346 => "Phoenix Smart IP43 Charger 24|16 (1+1)",
            0xA347 => "Phoenix Smart IP43 Charger 24|16 (3)",
            _ => "Unknown device",
        }
    }
}

pub enum StateOfOperation {
    Off = 0,
    LowPower = 1,
    Fault = 2,
    Bulk = 3,
    Absorption = 4,
    Float = 5,
    Storage = 6,
    Equalize = 7,
    Inverting = 9,
    PowerSupply = 11,
    StartingUp = 245,
    RepeatedAbsorption = 246,
    AutoEqualize = 247,
    BatterySafe = 248,
    ExternalControl = 252,
}

pub enum ErrorCode {
    NoError = 0,
    BatteryVoltageHigh = 2,
    ChargerTemperatureHigh = 17,
    ChargerCurrentHigh = 18,
    ChargerCurrentReversed = 19,
    BulkTimeLimit = 20,
    CurrentSensor = 21,
    TerminalTemperatureHigh = 26,
    Converter = 28,
    InputVoltageHigh = 33,
    InputCurrentHigh = 34,
    InputShutdownDueToBatteryVoltage = 38,
    InputShutdownDueToCurrentFlowWhileOff = 39,
    LostCommunication = 65,
    SynchronizedChargingConfiguration = 66,
    BmsConnectionLost = 67,
    NetworkMisconfigured = 68,
    FactoryCalibrationDataLost = 116,
    InvalidFirmware = 117,
    InvalidUserSettings = 119,
}

pub enum Mode {
    Charger = 1,
    Inverter = 2,
    Off = 4,
    Eco = 5,
    Hibernate = 253,
}

pub enum Mppt {
    Off = 0,
    VoltageOrCurrentLimited = 1,
    MpptTrackerActive = 2,
}

#[cfg(test)]
mod test {

}