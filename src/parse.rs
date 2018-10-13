use error::Error;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use toml;

#[derive(Clone)]
pub struct CliData {
    pub manifest_path: PathBuf,
    pub workspace_path: PathBuf,
    pub package_name: String,
    pub index_html_path: Option<PathBuf>,
}

impl CliData {
    pub fn sniff() -> Result<CliData, Error> {
        let pwd = env::current_dir().map_err(Error::Io)?;
        let manifest_path = find_manifest(&pwd)?;
        let manifest_str = fs::read_to_string(&manifest_path).map_err(Error::Io)?;
        let project_manifest: ProjectManifest =
            toml::from_str(&manifest_str).map_err(Error::ManifestParse)?;
        let workspace_path = find_workspace_for_manifest(&manifest_path);
        let index_html_path = find_index_html(manifest_path.parent().unwrap());
        Ok(CliData {
            manifest_path,
            workspace_path,
            package_name: project_manifest.package.name,
            index_html_path,
        })
    }

    pub fn wasm_file_path(&self, debug: bool) -> PathBuf {
        let mut file_path = self.target_path(debug);
        file_path.push(format!("{}.wasm", self.package_name.replace('-', "_")));
        file_path
    }

    pub fn target_path(&self, debug: bool) -> PathBuf {
        let mut target_path = self.workspace_path.clone();
        target_path.push("target/wasm32-unknown-unknown");
        if debug {
            target_path.push("debug");
        } else {
            target_path.push("release");
        }
        target_path
    }

    pub fn project_path(&self) -> &Path {
        self.manifest_path.parent().unwrap()
    }
}

#[derive(Deserialize)]
struct ProjectManifest {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

fn find_manifest(path: &Path) -> Result<PathBuf, Error> {
    let manifest = path.join("Cargo.toml");
    if fs::metadata(&manifest).is_ok() {
        Ok(manifest)
    } else {
        let path = path.parent().map(|parent| find_manifest(parent));
        match path {
            Some(val) => val,
            None => Err(Error::ManifestNotFound),
        }
    }
}

/// Recursively finds if there are any `Cargo.toml` in the parent directories
/// of the given manifest file.
fn find_workspace_for_manifest(path: &Path) -> PathBuf {
    fn find_workspace_recur(path: &Path) -> Option<PathBuf> {
        let manifest = path.join("Cargo.toml");
        if fs::metadata(&manifest).is_ok() {
            if let Some(parent) = path
                .parent()
                .and_then(|parent| find_workspace_recur(parent))
            {
                Some(parent)
            } else {
                Some(path.to_owned())
            }
        } else {
            path.parent()
                .and_then(|parent| find_workspace_recur(parent))
        }
    }

    find_workspace_recur(path.parent().unwrap()).unwrap()
}

fn find_index_html(path: &Path) -> Option<PathBuf> {
    let src = path.join("index.html");
    if src.is_file() {
        Some(src)
    } else {
        None
    }
}
