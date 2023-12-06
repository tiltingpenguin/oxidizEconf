use config::*;
use glob::glob;
use std::path::PathBuf;
use log;

fn get_conf_dirs(name: &str) -> Vec<PathBuf>{
    //let mut paths: Vec<PathBuf> = Vec::new();
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

    let paths = vec![etc_dir, 
        etc_subdir, 
        run_dir, 
        run_subdir, 
        usr_etcdir, 
        usr_etcsubdir, 
        usr_sharedir, 
        usr_sharesubdir, 
        usr_libdir, 
        usr_libsubdir
    ];
    paths
}

fn find_conf(mut paths: Vec<PathBuf>, name: &str, suffix: &str) -> String {
    for file in paths.iter_mut() {
        file.push(name);
        file.set_extension(suffix);
        let strpath = match file.to_str() {
            Some(str) => str,
            None => "",
        };
        for entry in glob(strpath).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    log::info!("Highest priority config file: {}", path.display());
                    return path.to_str().unwrap().to_string();
                },
                Err(e) => log::debug!("{}", e),
            }
        }
        log::info!("{:?}: file not found", file)
    }
    return "No file found".to_string();
}

pub fn read_config(name: &str, suffix: &str, _format: FileFormat) -> Result<Config, ConfigError> {
    let paths = get_conf_dirs(name);

    let configfile = find_conf(paths, name, suffix);

    let settings = Config::builder()
    .add_source(File::with_name(&configfile))
    .build();

    return settings;
}
/*
pub fn new_config(format: Config::FileFormat) {

}

pub fn merge_config(file1: Config::Config, file2: Config::Config) {

}

pub fn write_file(config: Config::Config) {

}

pub fn set_value(confg: Config, key: &str {

}

pub fn get_value(config: Config, key: &str) {

}

pub fn set_dirs(dirs: &[&str]) -> Config::ConfigError {

}

pub fn add_file_format(name: &str, delim: &str, comment: &str) -> Config::ConfigError{

}

*/
