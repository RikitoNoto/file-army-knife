
pub mod commands{
use crate::util::entry::entry::entry::{Entry, enumrate_file};

  #[tauri::command]
  pub async fn get_enumrate_files(path: String, pattern: String) -> Vec<String>{
    let mut vec: Vec<String> = vec![];
    match enumrate_file(path) {
        Ok(v) => {
          for match_entry in v{
            vec.push(match_entry.path);
          }
        },
        Err(_) => {}
    }
    println!("{}", vec.len());
    vec
  }
}

