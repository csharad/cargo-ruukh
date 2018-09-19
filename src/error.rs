use toml::de;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to parse the project manifest. {:?}", _0)]
    ManifestParse(de::Error),
}
