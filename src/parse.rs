use error::Error;
use toml;

pub struct CliData<'a> {
    pub package_name: &'a str,
}

impl<'a> CliData<'a> {
    fn parse(manifest: &str) -> Result<CliData<'_>, Error> {
        let manifest: ProjectManifest = toml::from_str(manifest).map_err(Error::ManifestParse)?;
        Ok(CliData {
            package_name: manifest.package.name,
        })
    }
}

#[derive(Deserialize)]
struct ProjectManifest<'a> {
    #[serde(borrow)]
    package: Package<'a>,
}

#[derive(Deserialize)]
struct Package<'a> {
    name: &'a str,
}
