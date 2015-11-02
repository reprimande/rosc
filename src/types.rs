use errors;

// see OSC Type Tag String: http://opensoundcontrol.org/spec-1_0
// padding: zero bytes (n*4)

#[derive(Debug)]
pub enum OscType {
    Int(i32),
    Float(f32),
    String(String),
    Blob(Vec<u8>),
    Time(u32, u32),
    // nonstandard argument types
    // ignore them if not implemented
    Long(i64),
    Double(f64),
    Char(u8),
    Color(u32), // byte-order: RGBA
    Midi(OscMidiType),
    True,
    False,
    Nil,
    Inf,
    Array(Vec<OscType>),
}


#[derive(Debug)]
pub struct OscMidiType {
    port: u8,
    status: u8,
    data1: u8, // maybe use an enum for data?
    data2: u8,
}

/// An *osc packet* can contain an *osc message* or a bundle of nested messages
/// which is called *osc bundle*.
#[derive(Debug)]
pub enum OscPacket {
    Message(OscMessage),
    Bundle(OscBundle),
}


#[derive(Debug)]
pub struct OscMessage {
    pub addr: String,
    pub args: Option<Vec<OscType>>,
}


#[derive(Debug)]
pub struct OscBundle {
    pub timetag: OscType,
    pub content: Vec<OscPacket>,
}

pub type OscResult<T> = Result<T, errors::OscError>;