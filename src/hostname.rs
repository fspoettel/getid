use rand::Rng;

static ADJECTIVES: [&str; 60] = [
    "adventurous",
    "awesome",
    "ancient",
    "autumn",
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
    "fluffy",
    "fresh",
    "frosty",
    "gentle",
    "gracious",
    "green",
    "groovy",
    "hidden",
    "icy",
    "indigo",
    "joyous",
    "lazy",
    "loud",
    "loving",
    "lucky",
    "lonely",
    "misty",
    "mysterious",
    "neon",
    "nice",
    "pretty",
    "purple",
    "quick",
    "quiet",
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
    "yellow",
];

static NOUNS: [&str; 60] = [
    "beach",
    "bear",
    "bird",
    "blossom",
    "breeze",
    "bumblebee",
    "butterfly",
    "cat",
    "cherry",
    "circle",
    "cloud",
    "crab",
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
    "meadow",
    "moon",
    "morning",
    "mountain",
    "mouse",
    "night",
    "ocean",
    "panda",
    "panther",
    "pear",
    "penguin",
    "pumpkin",
    "queen",
    "rain",
    "river",
    "rocket",
    "seal",
    "shadow",
    "ship",
    "sky",
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
    "wizard",
    "wolf",
];

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
