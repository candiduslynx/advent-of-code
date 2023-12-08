use std::collections::HashMap;

pub(crate) fn to_nodes(lines: Vec<String>) -> HashMap<String, (String, String)> {
    lines.iter().
        map(|s| s.trim()).filter(|s|!s.is_empty()).
        fold(HashMap::<String, (String, String)>::new(),|mut sum, s|{
            let parts: Vec<&str> = s.split(" = ").collect();
            assert_eq!(parts.len(), 2);

            let lr: Vec<&str> = parts[1]
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .collect();
            assert_eq!(lr.len(), 2);
            sum.insert(parts[0].to_string(), (lr[0].to_string(), lr[1].to_string()));
            sum
        })
}