use config::{builder::DefaultState, *};
use std::collections::HashMap;
use std::{error::Error, fmt::Debug, path::PathBuf};

#[derive(Debug, Clone)]
pub struct CfgBuilder {
    name: String,
    extension: String,
    project_name: Option<String>,
    path_override: Option<Vec<PathBuf>>,
    defaults: Option<HashMap<String, Value>>,
}

impl CfgBuilder {
    pub fn override_paths(&mut self, paths: Vec<PathBuf>) {
        // todo: check input
        self.path_override = Some(paths);
    }

    pub fn from_defaults(&mut self, defaults: HashMap<String, Value>) {
        // todo: check input
        self.defaults = Some(defaults);
    }

    pub fn set_project_name(&mut self, project_name: &str) {
        self.project_name = Some(project_name.to_owned());
    }

    pub fn build_config(self) -> Result<Config, ConfigError> {
        let paths = match self.path_override {
            Some(_) => self.path_override.clone().unwrap(),
            None => get_default_dirs(self.project_name),
        };

        let mut builder = read_config(&self.name, &self.extension, paths)?;

        if self.defaults.is_some() {
            for (key, value) in self.defaults.unwrap() {
                builder = builder.set_default(key, value)?;
            }
        }
        builder.build()
    }
}

pub fn new(name: &str, extension: &str) -> CfgBuilder {
    CfgBuilder {
        name: name.to_owned(),
        extension: extension.to_owned(),
        project_name: None,
        path_override: None,
        defaults: None,
    }
}

fn read_config(
    name: &str,
    extension: &str,
    paths: Vec<PathBuf>,
) -> Result<ConfigBuilder<DefaultState>, ConfigError> {
    let mut builder: ConfigBuilder<DefaultState> = Config::builder();
    let dropin_paths = paths.clone();
    let configfile = find_conf(paths, name, extension);

    if configfile.is_some() {
        builder = builder.add_source(File::from(configfile.unwrap()));
    }

    let dropin_files = find_dropins(dropin_paths, name);
    let dropins = read_dropins(dropin_files)?;

    for (key, val) in dropins {
        builder = builder.set_override(key, val)?;
    }

    Ok(builder)
}

fn get_default_dirs(project_name: Option<String>) -> Vec<PathBuf> {
    let etc_dir = PathBuf::from("/etc/");
    let run_dir = PathBuf::from("/run/");
    let usr_etcdir = PathBuf::from("/usr/etc/");
    let usr_sharedir = PathBuf::from("/usr/share/");
    let usr_libdir = PathBuf::from("/usr/lib/");

    let mut dirs = vec![etc_dir, run_dir, usr_etcdir, usr_sharedir, usr_libdir];
    if project_name.is_some() {
        let project_dir = project_name.unwrap();
        for d in dirs.iter_mut() {
            d.push(&project_dir);
        }
    }
    dirs
}

fn find_conf(mut paths: Vec<PathBuf>, name: &str, suffix: &str) -> Option<PathBuf> {
    for path in paths.iter_mut() {
        dbg!(&path);
        path.push(name);
        path.set_extension(suffix);
        if path.is_file() {
            let p = path.clone();
            return Some(p);
        }
    }
    log::info!("No main config file found, reading dropins");
    None
}

fn find_dropins(conf_dirs: Vec<PathBuf>, name: &str) -> Vec<PathBuf> {
    let mut dropin_paths: Vec<PathBuf> = vec![];
    for path in &conf_dirs {
        let ext1 = format!("{}.d", name);
        let ext2 = format!("{}.conf.d", name);
        let d = path.join(&ext1);
        let confd = path.join(&ext2);
        if d.is_dir() {
            dropin_paths.push(d);
        } else if confd.is_dir() {
            dropin_paths.push(confd);
        }
    }

    let mut dropin_list: Vec<PathBuf> = vec![];
    for path in dropin_paths {
        for entry in path.read_dir().expect("failed to read dir") {
            if let Ok(entry) = entry {
                if entry
                    .file_type()
                    .expect("dropin should have file type")
                    .is_file()
                {
                    dropin_list.push(entry.path());
                }
            }
        }
    }
    dropin_list.reverse();
    dropin_list
}

fn read_dropins(dropins: Vec<PathBuf>) -> Result<HashMap<String, Value>, ConfigError> {
    let mut dropin_cache: HashMap<String, Value> = HashMap::new();
    for d in dropins {
        let ext = match d.extension() {
            Some(ext) => ext.to_str().expect("Extension should be valid unicode"),
            None => "",
        };
        // try to parse any unknown file format as ini
        let f = match ext {
            "toml" | "json" | "yaml" | "yml" | "ini" | "ron" | "json5" => {
                File::from(d).collect()?
            }
            _ => {
                log::debug!("Unknown file format. Trying to parse as ini");
                File::new(d.to_str().unwrap(), FileFormat::Ini).collect()?
            }
        };
        for (key, val) in f.iter() {
            dropin_cache.insert(key.clone(), val.clone());
        }
    }

    Ok(dropin_cache)
}
