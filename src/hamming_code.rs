
//From https://github.com/JuxhinDB/hamming-code
pub fn encode(block: &mut u64) -> u64 {
    let len_power = 6;
    let len = 64;

    let mut code = 0u64;

    for i in 0..len {
        // Check if `i` is not a power of 2
        if (i != 0) && (i & (i - 1)) != 0 {
            code |= (0b1 << i) & *block as u64;
        } else {
            *block <<= 1;
        }
    }

    for i in 0..len_power {
        // If the parity check is odd, set the bit to 1 otherwise move on.
        if !parity(&code, i) {
            code |= 0b1 << (2usize.pow(i));
        } 
    }

    // Set the global parity
    code |= fast_parity(code);

    code
}

//From https://github.com/JuxhinDB/hamming-code
pub fn decode(code: &mut u64) -> u64 {
    let len_power = 6;
    let len = 64;

    let mut check = 0b0;

    for i in 0..len_power {
        if !parity(&code, i) {
            check |= 0b1 << i;
        }
    }
    
    // We have an error
    if check > 0b0 {
        println!("error at bit: {}", check);
        *code ^= 0b1 << check;
    }

    // Drop all parity bits
    let mut offset = 0;
    let mut decoded = 0b0;
    
    for i in 0..len {
        // Check if `i` is not a power of 2
        if (i != 0) && (i & (i - 1)) != 0 {
            decoded |= ((0b1 << i) & *code) >> offset;
        } else {
            offset += 1;
        }
    }

    decoded
}

/// Hacker's delight 2nd edition, p.96 
/// Henry S. Warren, Jr.
pub const fn fast_parity(code: u64) -> u64 {
    let mut y: u64 = code ^ (code >> 1);
    
    y ^= y << 2;
    y ^= y << 4;
    y ^= y << 8;
    y ^= y << 16;
    y ^= y << 32;

    0b1 & y
}
//From https://github.com/JuxhinDB/hamming-code
pub fn parity(code: &u64, i: u32) -> bool {
    let mut parity = true;
    let spread = 2u32.pow(i);
    let mut j = spread;

    while j < 64 - spread + 1 {
        for k in 0..spread {
            if (code & 0b1 << j + k) != 0b0 {
                parity = !parity;
            }
        }

        j += 2 * spread;
    }

    parity
}

//added
pub fn segment(data: &[u8]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut current_chunk: u64 = 0;
    let mut bits_in_chunk = 0;

    for &byte in data {
        for bit in 0..8 {
            if bits_in_chunk == 57 {
                result.push(current_chunk);
                current_chunk = 0;
                bits_in_chunk = 0;
            }
            current_chunk = (current_chunk << 1) | ((byte >> (7 - bit)) as u64 & 1);
            bits_in_chunk += 1;
        }
    }

    // Handle any remaining bits in the last chunk
    if bits_in_chunk > 0 {
        current_chunk <<= 57 - bits_in_chunk;  // Pad with zeros
        result.push(current_chunk);
    }

    result
}

pub fn merge(segments: &[u64]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut bit_buffer: u64 = 0;
    let mut bits_in_buffer = 0;

    for &segment in segments {
        let mut segment_bits = 57;
        while segment_bits > 0 && bits_in_buffer < 64 {
            bit_buffer = (bit_buffer << 1) | (segment >> (segment_bits - 1)) & 1;
            bits_in_buffer += 1;
            segment_bits -= 1;

            if bits_in_buffer == 8 {
                result.push(bit_buffer as u8);
                bit_buffer = 0;
                bits_in_buffer = 0;
            }
        }
    }

    // Handle any remaining bits in the bit buffer
    if bits_in_buffer > 0 {
        result.push((bit_buffer << (8 - bits_in_buffer)) as u8);
    }

    result
}
