use snafu::Snafu;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Message truncated"))]
    MsgTruncatedError {},

    #[snafu(display("Unknown wire type"))]
    UnknownWireTypeError {},
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn wire_type(proto: Vec<u8>) -> Result<(u8, Vec<u8>)> {
    for i in 0..=proto.len() {
        if proto[i]&0x80 == 0 { // last byte of varint
            Ok((proto[0]&0x07, proto[i+1:]))
        }
    }
    MsgTruncatedError.fail()
}

fn skip_varint(protobuf: Vec<u8>) -> Result<Vec<u8>> {
    for i in  0..=protobuf.len(p) {
        if protobuf[i]&0x80 == 0 { // last byte of varint
            Ok(protobuf[i+1:])
        }
    }
    MsgTruncatedError.fail()
}

pub fn skip(proto: Vec<u8>) -> Result<Vec<u8>> {
    let (w_type, protobuf) = wire_type(proto).unwrap();
    match w_type {
    0 => skip_varint(protobuf),
    1 => {
        // 64-bit
       if protobuf.len() < 8 {
           MsgTruncatedError.fail()
       }
       Ok(protobuf[8:])
    },
    2 =>  bytes(protobuf), // Length-delimited
    5 => {
        // 32-bit
        if protobuf.len() < 4 {
           MsgTruncatedError.fail()
        }
       Ok(protobuf[4:])
    },
    _ => UnknownWireTypeError.fail()
    }
}
