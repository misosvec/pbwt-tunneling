use utils::{encode_haplotypes, generate_haplotypes, write_to_file, read_haplotypes};
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("You need to specify input file and output file!");
    }
    let input_file = &args[1];
    let output_file = &args[2];
    let haplotypes = read_haplotypes(input_file);
    let encoded_haplotypes = encode_haplotypes(haplotypes);
    write_to_file(encoded_haplotypes.as_str(), output_file);
}