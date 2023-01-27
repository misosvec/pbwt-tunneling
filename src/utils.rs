use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

pub fn generate_haplotypes(count: usize, variable_sites: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut haplotypes: Vec<Vec<u8>> = vec![];

    for _ in 0..count {
        let mut haplotype: Vec<u8> = vec![];
        for _ in 0..variable_sites {
            haplotype.push(rng.gen_range(0..2));
        }
        haplotypes.push(haplotype);
    }
    return haplotypes;
}

pub fn write_to_file(data: &str, output_file: &str) {
    File::create(output_file).unwrap().write(data.as_ref()).unwrap();
}

pub fn encode_haplotypes(haplotypes: Vec<Vec<u8>>) -> String {
    let mut result: String = haplotypes
        .into_iter()
        .map(|haplotype| {
            let mut encoded_haplotype = haplotype.into_iter().enumerate().map(|(i, allele)| {
                let mut str = i.to_string();
                str.push(allele.to_string().chars().last().unwrap());
                return str;
            }).collect::<String>();
            encoded_haplotype += "#";
            return encoded_haplotype;
        }).collect();
    result.remove(result.len() - 1);
    return result;
}