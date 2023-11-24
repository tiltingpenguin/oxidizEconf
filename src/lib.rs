use config::*;
use glob::glob;
use log;

fn get_conf_dirs(name: &str, suffix: &str) -> Vec<String>{
    let etc_dir = format!("/etc/{}.{}", name, suffix);
    let run_dir = format!("/run/{}.{}", name, suffix);
    let usr_dir = format!("/usr/**/{}.{}", name, suffix);
    vec![etc_dir, run_dir, usr_dir]
}

fn find_conf(paths: Vec<String>) -> String {
    for file in paths {
        for entry in glob(&file).expect("Failed to read glob pattern") {
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

pub fn read_config(name: &str, suffix: &str, format: FileFormat) -> Result<Config, ConfigError> {
    let paths = get_conf_dirs(name, suffix);

    let configfile = find_conf(paths);

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
