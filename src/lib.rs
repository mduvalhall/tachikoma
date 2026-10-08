pub enum Packet {
    SpacePacket(SpacePacket),
}

pub enum SpacePacket {
    Command {
        header: SpacePacketHeader,
        data: Vec<u8>,
    },
    Telemetry {
        header: SpacePacketHeader,
        data: Vec<u8>,
    },
}

pub struct SpacePacketHeader {
    pub secondary_header_flag: bool,
    pub apid: u16,
    pub sequence_flags: SequenceFlags,
    pub sequence_count: u16,
}

pub enum SequenceFlags {
    Continuation = 0b00,
    First = 0b01,
    Last = 0b10,
    Unsegmented = 0b11,
}
