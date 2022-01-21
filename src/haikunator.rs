use rand::Rng;

static ADJECTIVES: [&str; 60] = [
    "adventurous",
    "awesome",
    "ancient",
    "autumn",
    "billowing",
    "blue",
    "calm",
    "charming",
    "clumsy",
    "cold",
    "cool",
    "cozy",
    "crimson",
    "delicate",
    "dreamy",
    "elusive",
    "enormous",
    "fancy",
    "floral",
    "flowing",
    "fluffy",
    "fresh",
    "frosty",
    "gentle",
    "gracious",
    "green",
    "groovy",
    "hidden",
    "icy",
    "joyous",
    "loud",
    "lucky",
    "misty",
    "mysterious",
    "neon",
    "nice",
    "pretty",
    "purple",
    "quick",
    "quiet",
    "red",
    "royal",
    "secret",
    "shiny",
    "sleepy",
    "slow",
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
    "wispy",
    "yellow",
];

static NOUNS: [&str; 60] = [
    "beach",
    "bear",
    "bird",
    "blossom",
    "boat",
    "breeze",
    "butterfly",
    "cat",
    "cherry",
    "circle",
    "cloud",
    "evening",
    "feather",
    "flower",
    "forest",
    "fox",
    "frog",
    "garden",
    "giant",
    "ghost",
    "haze",
    "lake",
    "leaf",
    "light",
    "meadow",
    "moon",
    "morning",
    "mountain",
    "mouse",
    "night",
    "ocean",
    "panda",
    "pear",
    "penguin",
    "pumpkin",
    "queen",
    "rain",
    "river",
    "rocket",
    "seal",
    "shadow",
    "sky",
    "snowflake",
    "snowman",
    "sound",
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
    "waterfall",
    "wildflower",
    "wizard",
    "wolf",
];

pub fn haikunator(token_length: usize) -> String {
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
