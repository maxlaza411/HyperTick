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


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct MboEvent {
        pub ts_event: u64,
        pub ts_delta: u32,
        pub order_id: u64,
        pub price:    u32,
        pub size:     u32,
        pub flags:    u8,
        pub action:   u8,
        pub side:     u8,
    }

    #[test]
    fn test_chunk_push_event() {
        let mut chunk = Chunk::new(123, 5, 100);

        // Check initial state
        assert_eq!(chunk.instrument_id, 123);
        assert_eq!(chunk.row_count, 0);
        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 100);
        assert_eq!(chunk.ts_delta.capacity(), 5);

        let e1 = MboEvent {
            ts_event: 100,
            ts_delta: 0,
            order_id: 1000,
            price:    5000,
            size:     10,
            flags:    1,
            action:   2,
            side:     0,
        };
        let e2 = MboEvent {
            ts_event: 105,
            ts_delta: 5,
            order_id: 1001,
            price:    5010,
            size:     15,
            flags:    0,
            action:   1,
            side:     1,
        };

        chunk.push_event(&e1);
        assert_eq!(chunk.row_count, 1);
        assert_eq!(chunk.ts_delta[0], 0);
        assert_eq!(chunk.order_id[0], 1000);
        assert_eq!(chunk.price[0],    5000);
        assert_eq!(chunk.size[0],     10);
        assert_eq!(chunk.flags[0],     1);
        assert_eq!(chunk.action[0],    2);
        assert_eq!(chunk.side[0],      0);

        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 100);

        chunk.push_event(&e2);
        assert_eq!(chunk.row_count, 2);
        assert_eq!(chunk.ts_delta[1], 5);
        assert_eq!(chunk.order_id[1], 1001);
        assert_eq!(chunk.price[1],    5010);
        assert_eq!(chunk.size[1],     15);
        assert_eq!(chunk.flags[1],     0);
        assert_eq!(chunk.action[1],    1);
        assert_eq!(chunk.side[1],      1);

        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 105);
    }

    #[test]
    fn test_out_of_order_ts() {
        let mut chunk = Chunk::new(999, 3, 200);

        let e1 = MboEvent {
            ts_event: 210,
            ts_delta: 10,
            order_id: 1,
            price:    100,
            size:     1,
            flags:    0,
            action:   0,
            side:     0,
        };
        chunk.push_event(&e1);

        let e2 = MboEvent {
            ts_event: 190,
            ts_delta: 0,
            order_id: 2,
            price:    99,
            size:     1,
            flags:    0,
            action:   1,
            side:     1,
        };
        chunk.push_event(&e2);

        assert_eq!(chunk.min_ts, 190);
        assert_eq!(chunk.max_ts, 210);
        assert_eq!(chunk.row_count, 2);
    }
}


// Event Arrives with timestamp T.
// Time Partition is computed from T (like bucket = T / bucket_size).
// Check if the chunk for (instrument, bucket) is still unsealed or is within the “grace period.”
// If yes, append it normally (in order or out-of-order, all good).
// If the chunk is sealed (or the grace period is expired), put the event into a “late delta store” for that (instrument, bucket).
// Query a time range → picks up data from the main chunk + the delta store for that time partition.