pub mod command {
  #[tauri::command]
  pub fn start(start_str: &str){
    println!("{}", start_str);
  }


}
