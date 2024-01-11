use config::{builder::DefaultState, *};
use std::collections::HashMap;
use std::{path::PathBuf, fmt::Debug, error::Error};
/*
struct Cfg {
    name: String,
    config: Config,
    path_override: Option<PathBuf>,
}
*/

fn get_default_dirs(name: &str) -> Vec<PathBuf> {
    let etc_dir = PathBuf::from("/etc/");
    let etc_subdir = etc_dir.join(name);
    let run_dir = PathBuf::from("/run/");
    let run_subdir = run_dir.join(name);
    let usr_etcdir = PathBuf::from("/usr/etc/");
    let usr_etcsubdir = usr_etcdir.join(name);
    let usr_sharedir = PathBuf::from("/usr/share/");
    let usr_sharesubdir = usr_sharedir.join(name);
    let usr_libdir = PathBuf::from("/usr/lib/");
    let usr_libsubdir = usr_libdir.join(name);

    vec![
        etc_dir,
        etc_subdir,
        run_dir,
        run_subdir,
        usr_etcdir,
        usr_etcsubdir,
        usr_sharedir,
        usr_sharesubdir,
        usr_libdir,
        usr_libsubdir,
    ]
}

fn find_conf(mut paths: Vec<PathBuf>, name: &str, suffix: &str) -> Option<PathBuf> {
    for path in paths.iter_mut() {
        path.set_file_name(name);
        path.set_extension(suffix);
        if path.is_file() {
            let p = path.clone();
            return Some(p);
        }
    }
    log::info!("No main config file found, reading dropins");
    None
}

fn find_dropins(conf_dirs: Vec<PathBuf>, name: &str /*, suffix: &str*/) -> Vec<PathBuf> {
    let mut dropin_paths: Vec<PathBuf> = vec![];
    for path in &conf_dirs {
        let ext1 = format!("{}.d", name);
        let ext2 = format!("{}.conf.d", name);
        let d = path.join(ext1);
        let confd = path.join(ext2);
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
    let mut dropin_map: HashMap<String, Value> = HashMap::new();
    for d in dropins {
        let ext = d.extension().unwrap();
        // try to parse any unknown file format as ini
        let f = match ext.to_str().unwrap() {
            "toml" | "json" | "yaml" | "yml" | "ini" | "ron" | "json5" => File::from(d).collect()?,
            _ => { 
                log::debug!("Unknown file format. Trying to parse as ini");
                File::new(d.to_str().unwrap(), FileFormat::Ini).collect()?
            },
        };
        for (key, val) in f.iter() {
            dropin_map.insert(key.clone(), val.clone());
        }
    }
    
    Ok(dropin_map)
}

pub fn read_config(name: &str, suffix: &str) -> Result<Config, ConfigError> {
    let mut builder: ConfigBuilder<DefaultState> = Config::builder();
    let paths = get_default_dirs(name);
    let dropin_paths = paths.clone();
    let configfile = find_conf(paths, name, suffix);

    if configfile.is_some() {
        builder = builder.add_source(File::from(configfile.unwrap()));
    }

    let dropin_files = find_dropins(dropin_paths, name);
    let dropins = read_dropins(dropin_files)?;

    for (key, val) in dropins {
        builder = builder.set_override(key, val)?;
    }

    builder.build()
}
/*
pub fn new_config(format: Config::FileFormat) {

}

pub fn merge_config(file1: Config::Config, file2: Config::Config) {

}

pub fn write_file(config: Config::Config, format: Config::FileFormat, path: PathBuf) -> Result<(), Error>{

}

pub fn set_value(confg: Config, key: &str {

}

pub fn get_value(config: Config, key: &str) {

}

pub fn set_dirs(dirs: &[&str]) -> Config::ConfigError {

}

pub fn add_file_format(name: &str, delim: &str, comment: &str) -> Config::ConfigError{

}

pub fn set_defaults(defaults: HashMap) {

}

*/
