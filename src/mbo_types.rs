

pub struct MboEvent {
    pub instrument_id: u32,
    pub ts_event: u64,     // nanosecond timestamp
    pub order_id: u64,
    pub price: u32,
    pub size: u32,
    pub flags: u8,
    pub action: u8,        // e.g. 0=ADD, 1=CANCEL, 2=EXECUTE
    pub side: u8,          // e.g. 0=bid, 1=ask
}