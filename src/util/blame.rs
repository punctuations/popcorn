pub struct Blame<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub version: &'a str,
    pub source: &'a str,
}

pub const BLAME: Blame = Blame {
    name: "🍿  Popcorn",
    description: "Say good-bye to complicated installation instructions.",
    version: "1.0.8",
    source: "https://github.com/punctuations/popcorn",
};
