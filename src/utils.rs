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