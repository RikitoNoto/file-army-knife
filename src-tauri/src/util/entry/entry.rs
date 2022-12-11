
pub mod entry{

  use std::io;
  use std::fs::{self, DirEntry};

  pub fn select_files(file_vec: Vec<DirEntry>, pattern: &str) -> Vec<DirEntry>
  {
    let vec: Vec<DirEntry> = Vec::new();
    vec
  }

  /// enumrate files in a directory.
  /// This function search recursively.
  ///
  pub fn enumrate_file(path: String) -> io::Result<Vec<DirEntry>>
  {
    let mut vec: Vec<DirEntry> = Vec::new();
    // read entry in directory.
    for entry in fs::read_dir(path)? {
      let entry: DirEntry = entry?;

      // if the entry is a directory.
      if entry.file_type()?.is_dir() {
        // enumrate children of entry.
        let mut children = enumrate_file(entry.path().into_os_string().into_string().unwrap())?;

        vec.append(&mut children);  // add children to vec.
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

  use std::fs::{self, DirEntry};
  use super::entry;

  speculate!{

    #[cfg(target_os = "windows")]
    use std::os::windows::fs::symlink_file as symlink;
    #[cfg(not(target_os = "windows"))]
    use std::os::unix::fs::symlink;

    before
    {
      remove_temp_dir(); // crean dir, because it do not clean when fail test.

      // create temp directory.
      if !Path::new("temp").exists(){
        create_dir("temp");
      }
    }

    after
    {
      remove_temp_dir();
    }

    /// remove temp dir include children.
    ///
    fn remove_temp_dir()
    {
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
    fn create_dir(path: &str)
    {
      if let Err(e) = fs::create_dir_all(path) {
        println!("{}",e);
        panic!("failed create dir.");
      }
    }

    /// create a file recursively.
    ///
    fn create_file(path: &str) -> fs::File
    {
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

    /// create a symbolic link recursively.
    ///
    fn create_symlink(link_info: (&str, &str))
    {
      let (src_path, link_path) = link_info;
      if let Some(p) = Path::new(link_path).parent(){
        if !p.exists() {
          create_dir(p.to_str().unwrap());
        }
      }

      match symlink(src_path, link_path){
        Ok(f) => f,
        Err(e) => {
          println!("{}", e);
          panic!("failed create file.");
        }
      }
    }

    /// create files in [`dir`].
    ///
    fn create_files_in(dir: &str, files: Vec<&str>)
    {
      for file in files{
        create_file(Path::new(dir).join(file).to_str().unwrap());
      }
    }

    describe "enumrate_file test"{
      use super::entry::enumrate_file;

      /// create fixture of [`ExploreInfo`] from path.
      ///
      fn explore_info_path(path: &str) -> String
      {
        path.to_string()
      }

      /// check return values of [`enumrate_file`].
      /// - vector length should  be expect length.
      /// - vector contents should  be expect content.
      fn check_enumrate_file(path: String , expect_len: usize, expect_path_vec: Vec<&str>)
      {
        match enumrate_file(path){
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

      it "should be get child files"{
        create_files_in("temp/parent", vec!["child1.c", "child2.c"]);
        check_enumrate_file(explore_info_path("temp"), 2, vec!["temp/parent/child1.c", "temp/parent/child2.c"]);
      }

      it "should be get grandson files"{
        create_files_in("temp/parent/child", vec!["grandson1.c", "grandson2.c"]);
        check_enumrate_file(explore_info_path("temp"), 2, vec!["temp/parent/child/grandson1.c", "temp/parent/child/grandson2.c"]);
      }

      it "should be not eternal loop by symbolic link"{
        create_symlink(("temp", "temp/temp.link"));
        check_enumrate_file(explore_info_path("temp"), 1, vec!["temp/temp.link"]);
      }

    }


    describe "select_files test"{
      use super::entry::select_files;
      before
      {
      }

      after
      {
      }

      it "should be get empty vec from empty vec and empty pattern"{
        let vec: Vec<DirEntry> = Vec::new();
        assert_eq!(select_files(vec, "").len(), 0);
      }

      it "should be get empty vec from a_content vec and empty pattern"{
        let vec: Vec<DirEntry> = Vec::new();
        // vec.push(DirEntry{

        // })
        assert_eq!(select_files(vec, "").len(), 0);
      }

    }

  }


}
