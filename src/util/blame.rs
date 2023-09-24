pub struct Blame<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub version: &'a str,
    pub source: &'a str,
}

pub const BLAME: Blame = Blame {
    name: "🍿  Popcorn",
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    source: env!("CARGO_PKG_REPOSITORY"),
};
