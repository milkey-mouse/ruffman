use std::{cmp::{self, Ordering}, collections::{binary_heap::BinaryHeap, HashMap}, io::{BufRead, Write}, fmt};

mod buf;
mod tree;
use tree::{ReverseWeightedNode};

pub fn analyze<'a>(symbol_size: usize, inputs: impl IntoIterator<Item = &'a mut (impl BufRead + 'a)>, mut output: impl Write) {
    let mut symbol_counts: HashMap<Vec<u8>, usize> = HashMap::new();

    // buffer.windows stops at the end of each chunk, but we don't want to miss
    // symbols straddling the boundaries of two chunks. so we "replay" the last
    // symbol_size-1 bytes on the next window scan. we need a contiguous &[u8]
    // to look up in the HashMap, so while we're draining the remainder we have
    // to pop bytes from the front and append from the new buffer one-by-one...
    let mut remainder = Vec::with_capacity(symbol_size);

    for mut input in inputs.into_iter() {
        loop {
            let buffer = input.fill_buf().unwrap(); // TODO: better error handling
            if buffer.is_empty() {
                break;
            }

            if !remainder.is_empty() {
                for i in 0..symbol_size {
                    remainder.remove(0);
                    remainder.push(buffer[i]);
                    if let Some(count) = symbol_counts.get_mut(&remainder) {
                        *count += 1;
                    } else {
                        symbol_counts.insert(remainder.clone(), 1);
                    }
                }
                remainder.clear();
            }

            for symbol in buffer.windows(symbol_size) {
                if let Some(count) = symbol_counts.get_mut(symbol) {
                    *count += 1;
                } else {
                    symbol_counts.insert(symbol.to_vec(), 1);
                }
            }

            // TODO: make remainder work if buffer is smaller than symbol size (unlikely, but possible)

            remainder.extend_from_slice(&buffer[buffer.len()-symbol_size+1..]);
            //assert_eq!(remainder.len(), (symbol_size-1));

            let length = buffer.len();
            input.consume(length);
        }
    }

    let mut top_symbols = symbol_counts
        .into_iter()
        .map(|(symbol, frequency)| ReverseWeightedNode::leaf(symbol, frequency))
        .collect::<BinaryHeap<_>>(); // TODO: roll your own min-heap

    loop {
        match (top_symbols.pop(), top_symbols.pop()) {
            (Some(a), Some(b)) => top_symbols.push(ReverseWeightedNode::parent(a, b)),
            (Some(a), None) => {
                writeln!(&mut output, "{}", a);
                break;
            },
            (None, Some(_)) => unreachable!(),
            (None, None) => {
                eprintln!("warning: all nones in match");
                // this can happen if there were no symbols at all
                // (i.e. the input files were empty)
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
