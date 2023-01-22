fn string_rotations(str: &str) -> Vec<String> {
    let mut rotations: Vec<String> = vec![];
    let str_len = str.len();
    for delimiter in 0..str_len {
        let (prefix, suffix) = str.split_at(delimiter);
        let result = [
            prefix.chars().rev().collect::<String>(),
            suffix.chars().rev().collect::<String>()
        ].join("");
        rotations.push(result.chars().rev().collect());
    }
    return rotations;
}

fn bwm(text: &str) -> Vec<String> {
    let mut rotations = string_rotations(text);
    rotations.sort();
    return rotations;
}

fn bwt(text: &str) -> String {
    let bmw = bwm(text);
    let mut last_col: Vec<char> = vec![];
    for r in bmw {
        last_col.push(r.chars().last().unwrap());
    }
    return String::from_iter(last_col);
}

#[cfg(test)]
mod tests {
    use crate::bwt::{bwm, bwt};

    #[test]
    fn test_bwm() {
        assert_eq!(bwm("mississippi$"), vec![
            "$mississippi",
            "i$mississipp",
            "ippi$mississ",
            "issippi$miss",
            "ississippi$m",
            "mississippi$",
            "pi$mississip",
            "ppi$mississi",
            "sippi$missis",
            "sissippi$mis",
            "ssippi$missi",
            "ssissippi$mi",
        ]);
    }

    #[test]
    fn test_bwt() {
        assert_eq!(bwt("mississippi$"), "ipssm$pissii");
    }
}
