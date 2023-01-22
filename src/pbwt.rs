fn build_prefix_array(haplotypes: Vec<Vec<u8>>) -> Vec<usize> {
    let mut ppa: Vec<usize> = (0..haplotypes.len()).collect();

    for k in 0..haplotypes[0].len() - 1 {
        let mut a: Vec<usize> = vec![];
        let mut b: Vec<usize> = vec![];

        for i in ppa {
            let haplotype = &haplotypes[i];
            let allele = haplotype[k];

            if allele == 0 {
                a.push(i);
            } else {
                b.push(i);
            }
        }
        ppa = [a, b].concat();
    }
    return ppa;
}

fn build_prefix_and_divergence_array(haplotypes: Vec<Vec<u8>>) -> Vec<usize> {
    let mut ppa: Vec<usize> = (0..haplotypes.len()).collect();
    let mut div: Vec<usize> = vec![0; haplotypes.len()];

    for k in 0..haplotypes[0].len()-1{
        let mut a: Vec<usize> = vec![];
        let mut b: Vec<usize> = vec![];
        let mut d: Vec<usize> = vec![];
        let mut e: Vec<usize> = vec![];
        let mut p: usize = k + 1;
        let mut q: usize = k + 1;

        for (index, match_start) in ppa.iter().zip(div.iter()) {
            let haplotype = &haplotypes[*index];
            let allele = haplotype[k];

            if *match_start > p {
                p = *match_start;
            }
            if *match_start > q {
                q = *match_start;
            }

            if allele == 0 {
                a.push(*index);
                d.push(p);
                p = 0;
            } else {
                b.push(*index);
                e.push(q);
                q = 0;
            }
        }
        ppa = [a, b].concat();
        div = [d, e].concat();
    }
    return div;
}

fn report_long_matches(
    haplotypes: Vec<Vec<u8>>,
    match_length: usize,
) -> Vec<(usize, Vec<usize>, Vec<usize>)> {
    let mut result: Vec<(usize, Vec<usize>, Vec<usize>)> = vec![];
    let mut ppa: Vec<usize> = (0..haplotypes.len()).collect();
    let mut div: Vec<usize> = vec![0; haplotypes.len()];

    for k in 0..haplotypes[0].len() {
        let mut a = vec![];
        let mut b = vec![];
        let mut d = vec![];
        let mut e = vec![];
        let mut p = k + 1;
        let mut q = k + 1;
        let mut ma = vec![];
        let mut mb = vec![];

        for (index, match_start) in ppa.iter().zip(div.iter()) {
            if *match_start as i32 > ((k as i32) - match_length as i32) {
                if !ma.is_empty() && !mb.is_empty() {
                    result.push((k, ma, mb));
                }
                ma = vec![];
                mb = vec![];
            }
            let haplotype = &haplotypes[*index];
            let allele = haplotype[k];

            if *match_start > p {
                p = *match_start;
            }
            if *match_start > q {
                q = *match_start;
            }

            if allele == 0 {
                a.push(*index);
                d.push(p);
                p = 0;
                ma.push(*index);
            } else {
                b.push(*index);
                e.push(q);
                q = 0;
                mb.push(*index)
            }
        }
        if !ma.is_empty() && !mb.is_empty() {
            result.push((k, ma, mb));
        }

        if k < (haplotypes[0].len() - 1) {
            ppa = [a, b].concat();
            div = [d, e].concat();
        }
    }
    return result;
}

fn report_set_maximal_matches(
    haplotypes: Vec<Vec<u8>>,
) -> Vec<(usize, usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize, usize)> = vec![];
    let haplotype_length = haplotypes[0].len();
    let mut ppa: Vec<usize> = (0..haplotypes.len()).collect();
    let mut div = vec![0; haplotypes.len()];

    for k in 0..haplotype_length {
        div.push((k + 1));

        for i in 0..haplotypes.len() {
            let mut m = i as i32 - 1;
            let mut n = i as u32 + 1;
            let mut match_continues = false;

            if div[i] <= div[i + 1] {
                while (m + 1) >= 0 && div[(m + 1) as usize] <= div[i] {
                    if m >= 0
                        && haplotypes[ppa[m as usize] as usize][k] == haplotypes[ppa[i] as usize][k]
                    {
                        match_continues = true;
                        break;
                    }
                    m -= 1;
                }
            }

            if match_continues {
                continue;
            }

            if div[i] >= div[i + 1] {
                while div[n as usize] <= div[i + 1] {
                    if haplotypes[ppa[n as usize] as usize][k] == haplotypes[ppa[i] as usize][k] {
                        match_continues = true;
                        break;
                    }
                    n += 1;
                }
            }

            if match_continues {
                continue;
            }

            for j in ((m + 1) as usize)..i as usize {
                if div[i] < k {
                    result.push((ppa[i], ppa[j], div[i], k))
                }
            }

            for j in i + 1..n as usize {
                if div[i + 1] < k {
                    result.push((ppa[i], ppa[j], div[i + 1], k))
                }
            }
        }
        if k < haplotype_length {
            let mut a = vec![];
            let mut b = vec![];
            let mut d = vec![];
            let mut e = vec![];
            let mut p = k + 1;
            let mut q = k + 1;

            for (index, match_start) in ppa.iter().zip(div.iter()) {
                let haplotype = &haplotypes[*index];
                let allele = haplotype[k];

                if *match_start > p {
                    p = *match_start;
                }
                if *match_start > q {
                    q = *match_start;
                }

                if allele == 0 {
                    a.push(*index);
                    d.push(p);
                    p = 0;
                } else {
                    b.push(*index);
                    e.push(q);
                    q = 0;
                }
            }
            ppa = [a, b].concat();
            div = [d, e].concat();
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::pbwt::build_prefix_array;
    use crate::pbwt::report_long_matches;
    use crate::pbwt::report_set_maximal_matches;
    use crate::pbwt::build_prefix_and_divergence_array;


    fn get_haplotypes() -> Vec<Vec<u8>> {
        return vec![
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0],
            vec![1, 1, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 1, 0],
        ];
    }


    #[test]
    fn test_prefix_array() {
        assert_eq!(build_prefix_array(get_haplotypes()), vec![4, 1, 6, 0, 5, 7, 3, 2]);
    }

    #[test]
    fn test_build_prefix_and_divergence_array() {
        assert_eq!(
            build_prefix_and_divergence_array(get_haplotypes()),
            vec![5, 2, 0, 4, 5, 4, 3, 1]
        );
    }

    #[test]
    fn test_report_matches() {
        assert_eq!(
            report_long_matches(get_haplotypes(), 3),
            vec![
                (4, vec![4], vec![5]),
                (4, vec![0], vec![7]),
                (5, vec![4], vec![1, 6]),
                (5, vec![3], vec![2]),
            ]
        );
    }

    #[test]
    fn test_report_set_maximal_matches() {
        assert_eq!(
            report_set_maximal_matches(get_haplotypes()),
            vec![
                (4, 0, 0, 1),
                (4, 3, 0, 1),
                (4, 7, 0, 1),
                (5, 1, 0, 1),
                (5, 2, 0, 1),
                (5, 6, 0, 1),
                (3, 0, 0, 2),
                (3, 7, 0, 2),
                (2, 1, 0, 2),
                (2, 6, 0, 2),
                (4, 5, 1, 4),
                (5, 4, 1, 4),
                (0, 7, 0, 4),
                (7, 0, 0, 4),
                (4, 1, 2, 5),
                (4, 6, 2, 5),
                (3, 2, 1, 5),
                (2, 3, 1, 5),
            ]
        );
    }
}
