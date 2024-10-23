use crate::core::core::get_os_env;
use std::fs;

/// Checks if a specific key exists in the configuration file `~/.witchrc`.
///
/// This function looks for a line in the `.witchrc` file that contains the specified `key`
/// in the format `key=value`. It returns `true` if the key exists, and `false` otherwise.
///
/// # Arguments
/// * `key` - A `&str` representing the key to search for in the `.witchrc` file.
///
/// # Returns
/// A `bool` indicating whether the specified key exists in the configuration file.
///
/// # Example
/// ```
/// let exists = readrc_exists("path_log_file");
/// if exists {
///     println!("Key exists in the configuration file.");
/// } else {
///     println!("Key does not exist.");
/// }
/// ```
pub fn readrc_exists(key: &str) -> bool {
    let fkey = format!("{}=", key);

    match fs::read_to_string("~/.witchrc") {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return true;
                }
            }
            false
        }
        Err(err) => {
            eprintln!("{}", err);
            return false;
        }
    }
}

/// Retrieves the value associated with a specific key from the configuration file `~/.witchrc`.
///
/// This function searches the `.witchrc` file for a line containing the specified `key` in
/// the format `key=value`. If the key is found, it returns the corresponding value as a `String`.
/// If the key does not exist or if the file cannot be read, it returns an empty string or
/// the string `"error"` respectively.
///
/// # Arguments
/// * `key` - A `&str` representing the key for which to retrieve the value from the `.witchrc` file.
///
/// # Returns
/// A `String` containing the value associated with the specified key if found. Returns an
/// empty string if the key does not exist, or `"error"` if the file cannot be read.
///
/// # Example
/// ```
/// let value = readrc_value("path_log_file");
/// if value.is_empty() {
///     println!("Key not found or has no value.");
/// } else {
///     println!("Value: {}", value);
/// }
/// ```
///
/// # Note
/// This function assumes that the `.witchrc` file is located in the user's home directory.
/// If the file cannot be read, an error message is printed along with the file path and an empty string
/// are returned
pub fn readrc_value(key: &str) -> String {
    let fkey = format!("{}=", key);
    let home = format!("{}.witchrc", get_os_env("HOME"));

    match fs::read_to_string(&home) {
        Ok(file) => {
            let lines: Vec<&str> = file.split("\n").collect();
            for line in lines {
                if line.contains(&fkey) {
                    return line.replace(&fkey, "");
                }
            }
            "".to_string()
        }
        Err(err) => {
            // println!("{} :: {}", err, home);
            return "".to_string();
        }
    }
}
