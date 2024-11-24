use std::path::PathBuf;

use xshell::Shell;

use crate::utils::project_root;

const DEV_VERSION: &str = "0.0.0";

pub fn run(sh: &Shell) -> anyhow::Result<()> {
    let version = match std::env::var("GITHUB_REF") {
        Ok(github_ref) if github_ref.starts_with("refs/tags/v") => {
            github_ref.trim_start_matches("refs/tags/v").to_owned()
        }
        _ => DEV_VERSION.to_owned(),
    };

    set_cargo_toml_version(sh, &version)?;
    set_pyproject_toml_version(sh, &version)?;
    set_editors_vscode_package_json_version(sh, &version)?;
    set_github_env_version(sh, &version)?;

    Ok(())
}

fn set_cargo_toml_version(sh: &Shell, version: &str) -> anyhow::Result<()> {
    let project_root = project_root();
    let mut patch = Patch::new(sh, project_root.join("Cargo.toml"))?;
    patch.replace(
        &format!(r#"version = "{}""#, DEV_VERSION),
        &format!(r#"version = "{}""#, version),
    );
    patch.commit(sh)?;
    Ok(())
}

fn set_pyproject_toml_version(sh: &Shell, version: &str) -> anyhow::Result<()> {
    let project_root = project_root();
    let mut patch = Patch::new(sh, project_root.join("pyproject.toml"))?;
    patch.replace(
        &format!(r#"version = "{}""#, DEV_VERSION),
        &format!(r#"version = "{}""#, version),
    );
    patch.commit(sh)?;
    Ok(())
}

fn set_editors_vscode_package_json_version(sh: &Shell, version: &str) -> anyhow::Result<()> {
    let mut patch = Patch::new(
        sh,
        project_root()
            .join("editors")
            .join("vscode")
            .join("package.json"),
    )?;

    patch.replace(
        &format!(r#""version": "{}""#, DEV_VERSION),
        &format!(r#""version": "{}""#, version),
    );

    patch.commit(sh)?;
    Ok(())
}

fn set_github_env_version(_: &Shell, version: &str) -> anyhow::Result<()> {
    if let Ok(github_env) = std::env::var("GITHUB_ENV") {
        std::env::set_var(
            "GITHUB_ENV",
            format!("{}\nTOMBI_VERSION={}", github_env, version),
        );
    }
    Ok(())
}

struct Patch {
    path: PathBuf,
    contents: String,
}

impl Patch {
    fn new(sh: &Shell, path: impl Into<PathBuf>) -> anyhow::Result<Patch> {
        let path = path.into();
        let contents = sh.read_file(&path)?;
        Ok(Patch { path, contents })
    }

    fn contents(&self) -> &str {
        &self.contents
    }

    fn replace(&mut self, from: &str, to: &str) -> &mut Patch {
        assert!(self.contents.contains(from));
        self.contents = self.contents.replace(from, to);
        self
    }

    fn commit(&self, sh: &Shell) -> anyhow::Result<()> {
        sh.write_file(&self.path, &self.contents)?;
        Ok(())
    }
}
