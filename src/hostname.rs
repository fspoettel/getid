use rand::Rng;

pub fn hostname(token_length: usize) -> String {
    let mut rng = rand::thread_rng();

    let i = rng.gen_range(0..ADJECTIVES.len());
    let j = rng.gen_range(0..NOUNS.len());
    let adjective = ADJECTIVES[i];
    let noun = NOUNS[j];
    let s = format!("{}-{}", adjective, noun);

    if token_length == 0 {
        return s;
    }

    let token = (0..token_length)
        .map(|_| {
            let d = rng.gen_range(0..10);
            char::from_digit(d, 10).unwrap()
        })
        .collect::<String>();

    format!("{}-{}", s, token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_n_token() {
        let res = hostname(4);

        let mut parts = res.split('-');
        assert_eq!(parts.clone().count(), 3);

        let adj = parts.next().unwrap();
        assert_eq!(ADJECTIVES.contains(&adj), true);

        let noun = parts.next().unwrap();
        assert_eq!(NOUNS.contains(&noun), true);

        let token = parts.next().unwrap();
        assert_eq!(token.len(), 4);
        assert_eq!(token.chars().all(|c| c.is_digit(10)), true);
    }

    #[test]
    fn test_with_zero_token() {
        let res = hostname(0);
        let parts = res.split('-');
        assert_eq!(parts.clone().count(), 2);
    }
}

const ADJECTIVES: &[&str; 64] = &[
    "adventurous",
    "awesome",
    "ancient",
    "autumn",
    "blue",
    "calm",
    "charming",
    "chill",
    "cold",
    "cool",
    "crimson",
    "decadent",
    "delicate",
    "dreamy",
    "elusive",
    "enormous",
    "evening",
    "fancy",
    "fearful",
    "floral",
    "fresh",
    "frosty",
    "gentle",
    "green",
    "groovy",
    "hidden",
    "icy",
    "indigo",
    "joyous",
    "lazy",
    "likeable",
    "lively",
    "loud",
    "lovely",
    "lucky",
    "lonely",
    "misty",
    "morning",
    "mysterious",
    "neon",
    "nice",
    "patient",
    "purple",
    "quick",
    "quiet",
    "restless",
    "secret",
    "shiny",
    "sleepy",
    "slow",
    "snazzy",
    "soft",
    "spring",
    "stunning",
    "summer",
    "superb",
    "sweet",
    "tiny",
    "travelling",
    "wandering",
    "whispering",
    "wild",
    "winter",
    "yellow",
    // waltzing
];

const NOUNS: &[&str; 64] = &[
    "beach",
    "bear",
    "bird",
    "blossom",
    "breeze",
    "bumblebee",
    "butterfly",
    "cat",
    "cherry",
    "cloud",
    "crab",
    "feather",
    "flower",
    "forest",
    "fox",
    "frog",
    "garden",
    "giant",
    "ghost",
    "haze",
    "hedgehog",
    "lake",
    "leaf",
    "meadow",
    "moon",
    "mountain",
    "mouse",
    "night",
    "ocean",
    "octopus",
    "panda",
    "panther",
    "pear",
    "pine",
    "penguin",
    "pumpkin",
    "queen",
    "raindrop",
    "river",
    "rocket",
    "shark",
    "sea",
    "seal",
    "shadow",
    "ship",
    "sky",
    "sloth",
    "snowflake",
    "snowman",
    "spider",
    "squid",
    "squirrel",
    "star",
    "storm",
    "sun",
    "sunset",
    "tourist",
    "tree",
    "water",
    "wave",
    "wildflower",
    "witch",
    "wizard",
    "wolf",
];

