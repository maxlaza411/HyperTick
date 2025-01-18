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


#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal mock for `crate::mbo_types::MboEvent` to test the Chunk.
    #[derive(Debug)]
    pub struct MboEvent {
        pub ts_event: u64,   // The absolute timestamp
        pub ts_delta: u32,   // A delta from chunk.min_ts (or something similar)
        pub order_id: u64,
        pub price:    u32,
        pub size:     u32,
        pub flags:    u8,
        pub action:   u8,
        pub side:     u8,
    }

    #[test]
    fn test_chunk_push_event() {
        // Create a chunk for instrument 123, capacity=5, first_ts=100
        let mut chunk = Chunk::new(123, 5, 100);

        // Check initial state
        assert_eq!(chunk.instrument_id, 123);
        assert_eq!(chunk.row_count, 0);
        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 100);
        assert_eq!(chunk.ts_delta.capacity(), 5);

        // Create some mock events
        let e1 = MboEvent {
            ts_event: 100,
            ts_delta: 0,    // ts_event - 100
            order_id: 1000,
            price:    5000,
            size:     10,
            flags:    1,
            action:   2,
            side:     0,
        };
        let e2 = MboEvent {
            ts_event: 105,
            ts_delta: 5,    // ts_event - 100
            order_id: 1001,
            price:    5010,
            size:     15,
            flags:    0,
            action:   1,
            side:     1,
        };

        // Push e1
        chunk.push_event(&e1);
        assert_eq!(chunk.row_count, 1);
        // The chunk columns should have 1 entry each
        assert_eq!(chunk.ts_delta[0], 0);
        assert_eq!(chunk.order_id[0], 1000);
        assert_eq!(chunk.price[0],    5000);
        assert_eq!(chunk.size[0],     10);
        assert_eq!(chunk.flags[0],     1);
        assert_eq!(chunk.action[0],    2);
        assert_eq!(chunk.side[0],      0);

        // min_ts/max_ts updated accordingly
        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 100);

        // Push e2
        chunk.push_event(&e2);
        assert_eq!(chunk.row_count, 2);
        // Now columns should have 2 entries
        assert_eq!(chunk.ts_delta[1], 5);
        assert_eq!(chunk.order_id[1], 1001);
        assert_eq!(chunk.price[1],    5010);
        assert_eq!(chunk.size[1],     15);
        assert_eq!(chunk.flags[1],     0);
        assert_eq!(chunk.action[1],    1);
        assert_eq!(chunk.side[1],      1);

        // min_ts should stay 100, max_ts should become 105
        assert_eq!(chunk.min_ts, 100);
        assert_eq!(chunk.max_ts, 105);
    }

    /// Example test showing if out-of-order timestamp arrives
    #[test]
    fn test_out_of_order_ts() {
        let mut chunk = Chunk::new(999, 3, 200);

        // e1 arrives with ts_event=210
        let e1 = MboEvent {
            ts_event: 210,
            ts_delta: 10,  // 210 - 200
            order_id: 1,
            price:    100,
            size:     1,
            flags:    0,
            action:   0,
            side:     0,
        };
        chunk.push_event(&e1);

        // e2 arrives with ts_event=190, which is < min_ts
        let e2 = MboEvent {
            ts_event: 190,
            // Typically you'd recalc ts_delta = 190 - 190 = 0 if you rebase min_ts
            // But let's just push what's given
            ts_delta: 0,
            order_id: 2,
            price:    99,
            size:     1,
            flags:    0,
            action:   1,
            side:     1,
        };
        chunk.push_event(&e2);

        // Now chunk.min_ts should be 190, max_ts = 210
        assert_eq!(chunk.min_ts, 190);
        assert_eq!(chunk.max_ts, 210);
        assert_eq!(chunk.row_count, 2);
    }
}