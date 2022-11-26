fn build_prefix_array(haplotypes: Vec<Vec<u32>>) -> Vec<u32> {
    let haplotype_length = haplotypes[0].len();
    let mut ppa: Vec<u32> = (0..haplotypes.len() as u32).collect();

    for k in 0..haplotype_length - 1 {
        let mut a: Vec<u32> = vec![];
        let mut b: Vec<u32> = vec![];

        for i in ppa {
            let haplotype = &haplotypes[i as usize];
            let allele = haplotype[k];

            if allele == 0 {
                a.push(i as u32);
            } else {
                b.push(i as u32);
            }
        }
        ppa = [a, b].concat();
    }
    return ppa;
}

#[cfg(test)]
mod tests {
    use crate::pbwt::build_prefix_array;

    #[test]
    fn test_prefix_array() {
        let haplotypes: Vec<Vec<u32>> = vec![
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0],
            vec![1, 1, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 1, 0],
        ];

        assert_eq!(build_prefix_array(haplotypes), vec![4, 1, 6, 0, 5, 7, 3, 2]);
    }
}
