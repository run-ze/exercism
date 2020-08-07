pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .enumerate()
        .map(|(i, _)| {
            if i < list.len() - 1 {
                format!("For want of a {} the {} was lost.", list[i], list[i + 1])
            } else {
                format!("And all for the want of a {}.", list[0])
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
