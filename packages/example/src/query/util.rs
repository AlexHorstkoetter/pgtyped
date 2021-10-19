pub struct ReplacementInterval<'a> {
    from: usize,
    to: usize,
    substitution: &'a str,
}

pub fn replace_intervals(input: &'static str, intervals: &mut Vec<ReplacementInterval>) -> String {
    intervals.sort_by(|a, b| a.from.cmp(&b.from));

    let mut result = String::new();
    let mut last_end: usize = 0;

    for interval in intervals {
        let preceding = &input[last_end..interval.from];

        result.push_str(preceding);
        result.push_str(interval.substitution);

        last_end = interval.to;
    }
    let remaining = &input[last_end..];
    result.push_str(remaining);

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_simple() {
        let query = "WHERE id = :id! AND";
        let mut intervals = vec![ReplacementInterval {
            from: 11,
            to: 15,
            substitution: "'uuid'",
        }];
        let spliced = replace_intervals(query, &mut intervals);
        assert_eq!(spliced, "WHERE id = 'uuid' AND")
    }

    #[test]
    fn test_replace_multiple() {
        let query = "Hallo, ☭, :a AND :b LOL";
        let mut intervals = vec![
            ReplacementInterval {
                from: 12,
                to: 14,
                substitution: "foo",
            },
            ReplacementInterval {
                from: 19,
                to: 21,
                substitution: "bar",
            },
        ];
        let spliced = replace_intervals(query, &mut intervals);
        assert_eq!(spliced, "Hallo, ☭, foo AND bar LOL")
    }
}
