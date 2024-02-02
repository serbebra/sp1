use crate::{
    precompiles::{poseidon2::Poseidon2ExternalEvent, PrecompileRuntime},
    runtime::Register,
};

use super::Poseidon2ExternalChip;

// TODO: I just copied and pasted these from sha as a starting point, so a lot will likely has to
// change.
impl<const N: usize> Poseidon2ExternalChip<N> {
    // TODO: How do I calculate this? I just copied and pasted these from sha as a starting point.
    pub const NUM_CYCLES: u32 = 8 * 4 + 64 * 4 + 8 * 4;

    pub fn execute(rt: &mut PrecompileRuntime) -> u32 {
        // Read `w_ptr` from register a0.
        let state_ptr = rt.register_unsafe(Register::X10);

        // Set the clock back to the original value and begin executing the
        // precompile.
        let saved_clk = rt.clk;
        let saved_state_ptr = state_ptr;
        let mut state_read_records = Vec::new();
        let mut state_write_records = Vec::new();

        // Execute the "initialize" phase.
        const H_START_IDX: u32 = 64;
        let mut hx = [0u32; 8];
        for i in 0..8 {
            let (record, value) = rt.mr(state_ptr + (H_START_IDX + i as u32) * 4);
            state_read_records.push(record);
            hx[i] = value;
            rt.clk += 4;
        }

        let mut input_state = Vec::new();
        // Execute the "compress" phase.
        let mut a = hx[0];
        let mut b = hx[1];
        let mut c = hx[2];
        let mut d = hx[3];
        let mut e = hx[4];
        let mut f = hx[5];
        let mut g = hx[6];
        let mut h = hx[7];
        // TODO: I think this is where I can read each element in the state and do stuff? Look into
        // this more.
        for i in 0..N {
            //        for i in 0..64 {
            // let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            // let ch = (e & f) ^ (!e & g);
            let (_record, w_i) = rt.mr(state_ptr + i * 4);
            input_state.push(w_i);
            // w_i_read_records.push(record);
            // let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(w_i);
            // let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            // let maj = (a & b) ^ (a & c) ^ (b & c);
            // let temp2 = s0.wrapping_add(maj);

            // h = g;
            // g = f;
            // f = e;
            // e = d.wrapping_add(temp1);
            // d = c;
            // c = b;
            // b = a;
            // a = temp1.wrapping_add(temp2);

            rt.clk += 4;
        }

        // Execute the "finalize" phase.
        let v = [a, b, c, d, e, f, g, h];
        for i in 0..8 {
            let record = rt.mw(
                state_ptr.wrapping_add((H_START_IDX + i as u32) * 4),
                hx[i].wrapping_add(v[i]),
            );
            state_write_records.push(record);
            rt.clk += 4;
        }

        // TODO: i need to put the poseidon2 event to the appropriate vector.
        // Push the Poseidon2 external event.
        rt.segment_mut()
            .poseidon2_external_events
            .push(Poseidon2ExternalEvent {
                clk: saved_clk,
                state_ptr: saved_state_ptr,
                state_reads: state_read_records.try_into().unwrap(),
                state_writes: state_write_records.try_into().unwrap(),
            });

        state_ptr
    }
}
