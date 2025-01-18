

pub struct Chunk {
    pub instrument_id: u16,

    // Relative to min ts
    pub ts_delta:      Vec<u32>,
    pub order_id:      Vec<u64>,
    pub price:         Vec<u32>,
    pub size:          Vec<u32>,
    pub flags:         Vec<u8>,
    pub action:        Vec<u8>,
    pub side:          Vec<u8>,

    pub row_count: usize,
    pub min_ts: u64,
    pub max_ts: u64,
}


impl Chunk {
    // Initialize an empty chunk with pre-allocated capacity.
    pub fn new(instrument_id: u16 capacity: usize, first_ts: u64) -> Self {
        Self {
            instrument_id,
            ts_delta:      Vec::with_capacity(capacity),
            order_id:      Vec::with_capacity(capacity),
            price:         Vec::with_capacity(capacity),
            size:          Vec::with_capacity(capacity),
            flags:         Vec::with_capacity(capacity),
            action:        Vec::with_capacity(capacity),
            side:          Vec::with_capacity(capacity),
            min_ts: first_ts,
            max_ts: first_ts,
        }
    }

    // Append a single MboEvent
    #[inline(always)]
    pub fn push_event(&mut self, evt: &crate::mbo_types::MboEvent) {
        self.instrument_id.push(evt.instrument_id);
        self.ts_delta.push(evt.ts_delta);
        self.order_id.push(evt.order_id);
        self.price.push(evt.price);
        self.size.push(evt.size);
        self.flags.push(evt.flags);
        self.action.push(evt.action);
        self.side.push(evt.side);

        // TODO: Out of order data
    }
}