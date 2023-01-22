const N: usize = 8;
const SIGMA: usize = 4;
const STRINGS: [&str; N] = ["GATTACAT", "TAGAGATA", "CATCACAT", "TACATACA", "GATAGATA", "TAAAGAGC", "ATTACCAT", "ACATTACT"];


fn alphabet_rank(c: char) -> usize {
    match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("Invalid character"),
    }
}

fn print_permutation(pi: [[usize; N + 1]; N]) {
    for i in 0..pi.len() { // 0..N
        for j in 0..pi[0].len() { // 0..N+1
            print!("{} ", pi[i][j]);
        }
        println!();
    }
}

fn right_to_left_radix_sort(strings: [&str; N]) -> [[usize; N + 1]; N] {
    let mut permutations: [[usize; N + 1]; N] = [[0; N + 1]; N];

    // fill last column with the values 0 - (N-1)
    for i in 0..N {
        permutations[i][N] = i;
    }


    for c_idx in (0..N).rev() {
        let mut freq: [usize; 4] = [0; SIGMA];

        // count the number of occurrences of each character in column
        for r_idx in 0..N {
            let character_rank = alphabet_rank(strings[r_idx].chars().nth(c_idx).unwrap());
            freq[character_rank] += 1;
        }

        let mut c = [0; SIGMA];

        // cumulative sum of lexicographically smaller characters
        // there is c[i] lexicographically smaller characters before c[i+1]
        for i in 1..SIGMA {
            c[i] = c[i - 1] + freq[i - 1];
        }

        let mut rank: [usize; 4] = [0; SIGMA];

        print_permutation(permutations);

        for r_idx in 0..N {
            let a = alphabet_rank(strings[permutations[r_idx][c_idx + 1]].chars().nth(c_idx).unwrap());
            permutations[c[a] + rank[a]][c_idx] = permutations[r_idx][c_idx + 1];
            rank[a] += 1;
        }
    }

    print_permutation(permutations);

    return permutations;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_to_left_radix_sort() {
        let expected: [[usize; N + 1]; N] = [
            [7, 5, 5, 6, 0, 3, 0, 1, 0],
            [6, 3, 7, 5, 2, 7, 2, 3, 1],
            [2, 1, 3, 1, 6, 5, 6, 4, 2],
            [4, 4, 1, 4, 5, 1, 3, 5, 3],
            [0, 2, 6, 3, 1, 4, 7, 0, 4],
            [5, 0, 4, 2, 4, 0, 5, 2, 5],
            [3, 7, 2, 0, 3, 2, 1, 6, 6],
            [1, 6, 0, 7, 7, 6, 4, 7, 7],
        ];
        assert_eq!(expected, right_to_left_radix_sort(STRINGS));
    }
}
