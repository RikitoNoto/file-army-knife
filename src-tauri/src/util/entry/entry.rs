
pub mod entry{

  use std::io;
  use std::fs::{self, DirEntry};

  pub struct ExploreInfo{
    pub path: String,
  }

  pub fn enumrate_file(info: ExploreInfo) -> io::Result<Vec<DirEntry>>{
    let mut vec: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(info.path)? {
      let entry: DirEntry = entry?;

      if entry.file_type()?.is_dir() {
        let dir_info = ExploreInfo{
          path: entry.path().into_os_string().into_string().unwrap()
        };
        let mut children = enumrate_file(dir_info)?;
        vec.append(&mut children);
      }
      else{
        vec.push(entry);
      }
    }
    Ok(vec)
  }
}


/// entry mod test.
///
/// [Attention]
/// This test use file system, so not recommend malti thread.
/// you should add `--test-threads=1` option to cargo command.
///
#[cfg(test)]
mod tests{
  extern crate speculate;

  use speculate::speculate;
  use std::path::Path;

  use std::fs;
  use super::entry;


  speculate!{
    use super::entry::ExploreInfo;
    use super::entry::enumrate_file;

    describe "enumrate_file test"{
      before{
        remove_temp_dir(); // crean dir, because it do not clean when fail test.

        // create temp directory.
        if !Path::new("temp").exists(){
          create_dir("temp");
        }
      }

      after{
        remove_temp_dir();
      }

      /// remove temp dir include children.
      ///
      fn remove_temp_dir(){
        if Path::new("temp").exists(){
          // remove temp directory.
          if let Err(e) = fs::remove_dir_all("temp") {
            println!("{}",e);
            panic!("failed remove dir.");
          }
        }
      }

      /// create dir recursively.
      ///
      fn create_dir(path: &str){
        if let Err(e) = fs::create_dir_all(path) {
          println!("{}",e);
          panic!("failed create dir.");
        }
      }

      /// create a file recursively.
      ///
      fn create_file(path: &str) -> fs::File{
        if let Some(p) = Path::new(path).parent(){
          if !p.exists() {
            create_dir(p.to_str().unwrap());
          }
        }

        match fs::File::create(path){
          Ok(f) => f,
          Err(e) => {
            println!("{}", e);
            panic!("failed create file.");
          }
        }
      }

      /// create files in [`dir`].
      ///
      fn create_files_in(dir: &str, files: Vec<&str>){
        for file in files{
          create_file(Path::new(dir).join(file).to_str().unwrap());
        }
      }

      /// create fixture of [`ExploreInfo`] from path.
      ///
      fn explore_info_path(path: &str) -> ExploreInfo{
        ExploreInfo{
          path: path.to_string(),
        }
      }

      /// check return values of [`enumrate_file`].
      /// - vector length should  be expect length.
      /// - vector contents should  be expect content.
      fn check_enumrate_file(info: ExploreInfo , expect_len: usize, expect_path_vec: Vec<&str>){
        match enumrate_file(info){
          Ok(v) => {
            assert_eq!(v.len(), expect_len);

            for entry in v{
              assert!(expect_path_vec.contains(&(entry.path().into_os_string().into_string().unwrap().as_str())));
            }
          },
          Err(e) => {
            println!("{}", e);
            panic!("fail enumrate file.");
          }
        };
      }



      it "should be get no file when dir is empty"{
        check_enumrate_file(explore_info_path("temp"), 0, vec![]);
      }

      it "should be get one file when dir has a file"{
        create_files_in("temp", vec!["temp.a"]);
        check_enumrate_file(explore_info_path("temp"), 1, vec!["temp/temp.a"]);
      }

      it "should be get two file when dir has two files"{
        create_files_in("temp", vec!["temp1.a", "temp2.a"]);
        check_enumrate_file(explore_info_path("temp"), 2, vec!["temp/temp1.a", "temp/temp2.a"]);
      }

      it "should be get a child file"{
        create_files_in("temp/parent", vec!["child.c"]);
        check_enumrate_file(explore_info_path("temp"), 1, vec!["temp/parent/child.c"]);
      }

      //TODO: shoud be not get symlink.

    }

  }


}
