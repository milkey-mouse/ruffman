use std::{env::args_os, io::{Read, stdin, stdout}};

fn main() {
    let mut vec = Vec::new();
    stdin().lock().read_to_end(&mut vec);
    ruffman::analyze(1, &mut [&vec[..]], stdout().lock());
}
