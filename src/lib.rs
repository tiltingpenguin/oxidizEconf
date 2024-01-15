use config::{builder::DefaultState, *};
use std::collections::HashMap;
use std::{path::PathBuf, fmt::Debug, error::Error};

#[derive(Debug, Clone)]
pub struct Cfg {
    name: String,
    extension: String,
    config: Config,
    path_override: Option<Vec<PathBuf>>,
}

pub enum ValueType {
    String,
    Int,
    Float,
    Bool,
    Table,
    Array,
}

impl Cfg {
    pub fn from_config(name: &str, suffix: &str) -> Result<Self, ConfigError> {
        let placeholder = Config::builder().build().expect("Should be able to construct empty config object");
        let mut cfg = Self {
            name: name.to_owned(),
            extension: suffix.to_owned(),
            config: placeholder,
            path_override: None,
        };
        cfg.read_config()?;
        Ok(cfg)
    }

    pub fn read_config(&mut self) -> Result<(),ConfigError> {
        let mut builder: ConfigBuilder<DefaultState> = Config::builder();
        let paths = match self.path_override {
            Some(_) => self.path_override.clone().unwrap(),
            None => get_default_dirs(),
        };
        let dropin_paths = paths.clone();
        let configfile = find_conf(paths, &self.name, &self.extension);
    
        if configfile.is_some() {
            builder = builder.add_source(File::from(configfile.unwrap()));
        }
    
        let dropin_files = find_dropins(dropin_paths, &self.name);
        let dropins = read_dropins(dropin_files)?;
    
        for (key, val) in dropins {
            builder = builder.set_override(key, val)?;
        }
        self.config = builder.build()?;

        Ok(())
    }

    pub fn override_paths(&mut self, paths: Vec<PathBuf>) {
        self.path_override = Some(paths);
    }

    pub fn get_value(&self, key: &str, rettype: ValueType) -> Result<ValueKind, ConfigError> {
        let val = match rettype {
            ValueType::String => ValueKind::String(self.config.get_string(key)?),
            ValueType::Int => ValueKind::I64(self.config.get_int(key)?),
            ValueType::Float => ValueKind::Float(self.config.get_float(key)?),
            ValueType::Bool => ValueKind::Boolean(self.config.get_bool(key)?),
            ValueType::Table => ValueKind::Table(self.config.get_table(key)?),
            ValueType::Array => ValueKind::Array(self.config.get_array(key)?),
        };
        Ok(val)
    }

}


fn get_default_dirs() -> Vec<PathBuf> {
    let etc_dir = PathBuf::from("/etc/");
    let run_dir = PathBuf::from("/run/");
    let usr_etcdir = PathBuf::from("/usr/etc/");
    let usr_sharedir = PathBuf::from("/usr/share/");
    let usr_libdir = PathBuf::from("/usr/lib/");

    vec![
        etc_dir,
        run_dir,
        usr_etcdir,
        usr_sharedir,
        usr_libdir,
    ]
}

fn find_conf(mut paths: Vec<PathBuf>, name: &str, suffix: &str) -> Option<PathBuf> {
    for path in paths.iter_mut() {
        let mut subpath = path.join(name);
        path.push(name);
        path.set_extension(suffix);
        if path.is_file() {
            let p = path.clone();
            return Some(p);
        }
        subpath.push(name);
        subpath.set_extension(suffix);
        if subpath.is_file() {
            return Some(subpath);
        }
    }
    log::info!("No main config file found, reading dropins");
    None
}

fn find_dropins(conf_dirs: Vec<PathBuf>, name: &str) -> Vec<PathBuf> {
    let mut dropin_paths: Vec<PathBuf> = vec![];
    for path in &conf_dirs {
        let subpath = path.join(name);
        let ext1 = format!("{}.d", name);
        let ext2 = format!("{}.conf.d", name);
        let d = path.join(&ext1);
        let confd = path.join(&ext2);
        let subd = subpath.join(&ext1);
        let subconfd = subpath.join(&ext2);
        if d.is_dir() {
            dropin_paths.push(d);
        } else if confd.is_dir() {
            dropin_paths.push(confd);
        } else if subd.is_dir() {
            dropin_paths.push(subd);
        } else if subconfd.is_dir() {
            dropin_paths.push(subconfd);
        }
    }
    
    let mut dropin_list: Vec<PathBuf> = vec![];
    for path in dropin_paths {
        for entry in path.read_dir().expect("failed to read dir") {
            if let Ok(entry) = entry {
                if entry.file_type().expect("dropin should have file type").is_file() {
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
            "toml" | "json" | "yaml" | "yml" | "ini" | "ron" | "json5" => File::from(d).collect()?,
            _ => { 
                log::debug!("Unknown file format. Trying to parse as ini");
                File::new(d.to_str().unwrap(), FileFormat::Ini).collect()?
            },
        };
        for (key, val) in f.iter() {
            dropin_cache.insert(key.clone(), val.clone());
        }
    }
    
    Ok(dropin_cache)
}
/*
pub fn merge_config(file1: Config::Config, file2: Config::Config) {

}

pub fn write_file(config: Config::Config, format: Config::FileFormat, path: PathBuf) -> Result<(), Error>{

}

pub fn set_value(confg: Config, key: &str {

}

pub fn get_value(config: Config, key: &str) {

}

pub fn add_file_format(name: &str, delim: &str, comment: &str) -> Config::ConfigError{

}

pub fn set_defaults(defaults: HashMap) {

}

*/
