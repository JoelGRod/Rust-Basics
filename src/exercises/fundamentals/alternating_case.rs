/*
    Define String.prototype.toAlternatingCase (or a similar function/method 
    such as to_alternating_case/toAlternatingCase/ToAlternatingCase in your 
    selected language; see the initial solution for details) such that each 
    lowercase letter becomes uppercase and each uppercase letter becomes 
    lowercase. For example
*/

fn to_alternating_case(s: &str) -> String {
    let mut result: String = String::with_capacity(s.len());

    for c in s.to_owned().chars() {
        if c.is_lowercase() {
            let upper: char = c.to_uppercase().next().unwrap();
            result.push(upper);
        } else {
            let lower: char = c.to_lowercase().next().unwrap();
            result.push(lower);
        }
    }
    result
}

fn to_alternating_case_two(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_uppercase() {
            result.extend(c.to_lowercase())
        } else {
            result.extend(c.to_uppercase())
        }
    }
    result
}

fn to_alternating_case_three(s: &str) -> String {
    s.chars().map(|c| {
        match c {
            c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
            c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
            _ => c
        }
    }).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_alternate_chars_case() {
        assert_eq!(to_alternating_case("HeLlO wOrLd"), "hElLo WoRlD");
        assert_eq!(to_alternating_case_two("HeLlO wOrLd"), "hElLo WoRlD");
        assert_eq!(to_alternating_case_three("HeLlO wOrLd"), "hElLo WoRlD");
    }

    #[test]
    fn must_alternate_chars_case_two() {
        assert_eq!(to_alternating_case("altERnaTIng cAsE"), "ALTerNAtiNG CaSe");
        assert_eq!(to_alternating_case_two("altERnaTIng cAsE"), "ALTerNAtiNG CaSe");
        assert_eq!(to_alternating_case_three("altERnaTIng cAsE"), "ALTerNAtiNG CaSe");
    }
}
