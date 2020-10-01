pub fn raindrops(n: u32) -> String {
    let mut raindrop_sound = "".to_string();

    if n % 3 == 0 {
        raindrop_sound.push_str("Pling");
    }
    if n % 5 == 0 {
        raindrop_sound.push_str("Plang");
    }
    if n % 7 == 0 {
        raindrop_sound.push_str("Plong");
    }

    if raindrop_sound.is_empty() {n.to_string()} else {raindrop_sound}
}
