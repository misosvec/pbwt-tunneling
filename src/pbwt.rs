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

fn build_prefix_and_divergence_array(haplotypes: Vec<Vec<u32>>) -> Vec<u32> {
    let haplotype_length = haplotypes[0].len();
    let mut ppa: Vec<u32> = (0..haplotypes.len() as u32).collect();
    let mut div: Vec<u32> = vec![0; haplotypes.len()];

    for k in 0..haplotype_length - 1 {
        let mut a: Vec<u32> = vec![];
        let mut b: Vec<u32> = vec![];
        let mut d: Vec<u32> = vec![];
        let mut e: Vec<u32> = vec![];
        let mut p: u32 = k as u32 + 1;
        let mut q: u32 = k as u32 + 1;

        for (index, match_start) in ppa.iter().zip(div.iter()) {
            let haplotype = &haplotypes[*index as usize];
            let allele = haplotype[k];

            if *match_start > p {
                p = *match_start;
            }
            if *match_start > q {
                q = *match_start;
            }

            if allele == 0 {
                a.push(*index as u32);
                d.push(p);
                p = 0;
            } else {
                b.push(*index as u32);
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
    haplotypes: Vec<Vec<u32>>,
    match_length: u32,
) -> Vec<(usize, Vec<u32>, Vec<u32>)> {
    let mut result: Vec<(usize, Vec<u32>, Vec<u32>)> = vec![];
    let haplotype_length = haplotypes[0].len();
    let mut ppa: Vec<u32> = (0..haplotypes.len() as u32).collect();
    let mut div: Vec<u32> = vec![0; haplotypes.len()];

    for k in 0..haplotype_length {
        let mut a: Vec<u32> = vec![];
        let mut b: Vec<u32> = vec![];
        let mut d: Vec<u32> = vec![];
        let mut e: Vec<u32> = vec![];
        let mut p: u32 = k as u32 + 1;
        let mut q: u32 = k as u32 + 1;
        let mut ma: Vec<u32> = vec![];
        let mut mb: Vec<u32> = vec![];

        for (index, match_start) in ppa.iter().zip(div.iter()) {
            if *match_start as i32 > ((k as i32) - match_length as i32) {
                if !ma.is_empty() && !mb.is_empty() {
                    result.push((k, ma, mb));
                }
                ma = vec![];
                mb = vec![];
            }
            let haplotype = &haplotypes[*index as usize];
            let allele = haplotype[k];

            if *match_start > p {
                p = *match_start;
            }
            if *match_start > q {
                q = *match_start;
            }

            if allele == 0 {
                a.push(*index as u32);
                d.push(p);
                p = 0;
                ma.push(*index as u32);
            } else {
                b.push(*index as u32);
                e.push(q);
                q = 0;
                mb.push(*index as u32)
            }
        }
        if !ma.is_empty() && !mb.is_empty() {
            result.push((k, ma, mb));
        }

        if k < (haplotype_length - (1 as usize)) {
            ppa = [a, b].concat();
            div = [d, e].concat();
        }
    }
    return result;
}

fn report_set_maximal_matches(
    haplotypes: Vec<Vec<u32>>,
    match_length: u32,
) -> Vec<(u32, u32, u32, u32)> {
    let mut result: Vec<(u32, u32, u32, u32)> = vec![];
    let haplotype_length = haplotypes[0].len();
    let mut ppa: Vec<u32> = (0..haplotypes.len() as u32).collect();
    let mut div: Vec<u32> = vec![0; haplotypes.len()];

    for k in 0..haplotype_length {
        div.push((k + 1) as u32);

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
                if div[i] < k as u32 {
                    result.push((ppa[i], ppa[j], div[i], k as u32))
                }
            }

            for j in i + 1..n as usize {
                if div[i + 1] < k as u32 {
                    result.push((ppa[i], ppa[j], div[i + 1], k as u32))
                }
            }
        }
        if k < haplotype_length {
            let mut a: Vec<u32> = vec![];
            let mut b: Vec<u32> = vec![];
            let mut d: Vec<u32> = vec![];
            let mut e: Vec<u32> = vec![];
            let mut p: u32 = k as u32 + 1;
            let mut q: u32 = k as u32 + 1;

            for (index, match_start) in ppa.iter().zip(div.iter()) {
                let haplotype = &haplotypes[*index as usize];
                let allele = haplotype[k];

                if *match_start > p {
                    p = *match_start;
                }
                if *match_start > q {
                    q = *match_start;
                }

                if allele == 0 {
                    a.push(*index as u32);
                    d.push(p);
                    p = 0;
                } else {
                    b.push(*index as u32);
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
    use crate::pbwt::build_prefix_and_divergence_array;
    use crate::pbwt::build_prefix_array;
    use crate::pbwt::report_long_matches;
    use crate::pbwt::report_set_maximal_matches;

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
    #[test]
    fn test_build_prefix_and_divergence_array() {
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
        assert_eq!(
            build_prefix_and_divergence_array(haplotypes),
            vec![5, 2, 0, 4, 5, 4, 3, 1]
        );
    }
    #[test]
    fn test_report_matches() {
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
        assert_eq!(
            report_long_matches(haplotypes, 3),
            vec![
                (4, vec![4], vec![5]),
                (4, vec![0], vec![7]),
                (5, vec![4], vec![1, 6]),
                (5, vec![3], vec![2])
            ]
        );
    }

    #[test]
    fn test_report_set_maximal_matches() {
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
        assert_eq!(
            report_set_maximal_matches(haplotypes, 3),
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
                (2, 3, 1, 5)
            ]
        );
    }
}
