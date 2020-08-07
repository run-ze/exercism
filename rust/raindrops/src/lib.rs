pub fn raindrops(n: u32) -> String {
    let mut sounds: String = "".into();

    struct Action<'a>(u32, &'a str);
    let actions = vec![Action(3, "Pling"), Action(5, "Plang"), Action(7, "Plong")];
    for Action(factor, sound) in actions.iter() {
        if n % factor == 0 {
            sounds.push_str(sound)
        }
    }

    if sounds.is_empty() {
        sounds.push_str(&n.to_string())
    }
    sounds
}
