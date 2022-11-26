
#[tauri::command]
pub fn input_start_string(startStr: &str){
  println!("{}", startStr);
}
