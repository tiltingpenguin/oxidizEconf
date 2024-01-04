use config::{builder::DefaultState, *};
use std::collections::HashMap;
use std::path::PathBuf;

fn get_conf_dirs(name: &str) -> Vec<PathBuf> {
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

fn find_dropins(conf_dirs: Vec<PathBuf>, name: &str /*, suffix: &str*/) -> Vec<PathBuf> {
    let mut dropin_paths: Vec<PathBuf> = vec![];
    for path in &conf_dirs {
        let base = path.join(name);
        let d = base.join(".d");
        let confd = base.join(".conf.d");
        if d.is_dir() {
            dropin_paths.push(d);
        } else if confd.is_dir() {
            dropin_paths.push(confd);
        }
    }

    dropin_paths
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
    return None;
}

pub fn read_config(name: &str, suffix: &str, format: FileFormat) -> Result<Config, ConfigError> {
    let paths = get_conf_dirs(name);
    let dropin_paths = paths.clone();

    let configfile = find_conf(paths, name, suffix);
    let dropins = find_dropins(dropin_paths, name);

    let mut builder: ConfigBuilder<DefaultState> = Config::builder();

    if configfile.is_some() {
        let s = File::new(configfile.unwrap().to_str().unwrap(), format);
        builder = builder.add_source(s);
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
