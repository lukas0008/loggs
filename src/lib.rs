use chrono::Local;
use std::{
  collections::HashMap,
  io::Write,
  path::Path,
  sync::{Arc, Mutex},
};

#[derive(Debug)]
struct LoggerState {
  save_location: Box<Path>,
  hash_map: HashMap<String, Arc<Mutex<Vec<String>>>>,
}

/// Clonable logger that can save logs to a directory.
#[derive(Clone, Debug)]
pub struct Logger {
  state: Arc<Mutex<LoggerState>>,
}
impl Logger {
  /// Create a new logger with the default save location
  /// Linux - /var/log/{app_name}
  /// Windows - %AppData%/{app_name}/Logs
  pub fn new_default_location(app_name: impl Into<String>) -> Self {
    let app_name = app_name.into();
    #[cfg(target_os = "linux")]
    let path = format!("/var/log/{}", app_name);
    #[cfg(target_os = "windows")]
    let save_location = Path::new(std::env::var("APPDATA").unwrap().as_str())
      .join(app_name)
      .join("Logs");
    Logger::new(save_location.as_path())
  }

  /// Create a new logger with a custom save location
  pub fn new(save_location: &Path) -> Self {
    if !save_location.exists() {
      std::fs::create_dir_all(save_location).unwrap();
    }
    let state = LoggerState {
      save_location: Box::from(save_location),
      hash_map: HashMap::new(),
    };
    Self {
      state: Arc::new(Mutex::new(state)),
    }
  }

  /// Saves all logs inside of the before specified log directory inside of a directory with the current time and clears the log buffer
  pub fn save_logs(&self) {
    Logger::save_logs_internal(self.state.clone());
  }

  /// Adds a log to the log buffer
  pub fn log(&self, key: impl Into<String>, log: impl Into<String>) {
    let key = key.into();
    let log = log.into();
    Logger::log_internal(self.state.clone(), key, format!("{}\n", log));
  }

  fn log_internal(this: Arc<Mutex<LoggerState>>, key: String, log: String) {
    let mut state = this.lock().unwrap();
    if !state.hash_map.contains_key(&key) {
      state
        .hash_map
        .insert(key, Arc::from(Mutex::from(Vec::from([log]))));
    } else {
      let mut hm = state.hash_map[&key].lock().unwrap();
      hm.push(log);
    }
  }

  fn save_logs_internal(this: Arc<Mutex<LoggerState>>) {
    let mut state = this.lock().unwrap();
    let date = Local::now().format("%Y.%m.%d %H-%M-%S").to_string();
    let location = state.save_location.clone().join(format!("{}", date));
    std::fs::create_dir(location.clone()).unwrap();
    for key in state.hash_map.keys() {
      let file_location = location.join(format!("{}.log.txt", key));
      println!("{:?}", file_location);
      let mut file = std::fs::File::create(file_location).unwrap();
      let hm = state.hash_map.get(key).unwrap().lock().unwrap();
      for line in hm.clone() {
        file.write_all(line.as_bytes()).unwrap();
      }
    }

    state.hash_map = HashMap::new();
  }

  /// Save logs on panic
  pub fn save_on_panic(&self) {
    let state = self.state.clone();
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
      Logger::save_logs_internal(state.clone());
      default_hook(info);
    }));
  }
}
