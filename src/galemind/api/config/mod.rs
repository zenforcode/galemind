#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub service_host: String,
    pub service_port: int,
}

trait Configurator {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppConfig, Box<dyn std::error::Error>>;
}

