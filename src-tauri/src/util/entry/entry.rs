
pub mod entry{

  use std::io;
  use std::fs::{self, DirEntry};

  pub struct ExploreInfo{
    pub path: String,
  }

  pub fn enumrate_file(info: ExploreInfo) -> io::Result<Vec<DirEntry>>{
    let mut vec: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(info.path)? {
      let entry = entry?;
      vec.push(entry);
    }
    Ok(vec)
  }
}


#[cfg(test)]
mod tests{
  extern crate speculate;

  use speculate::speculate;
  use rstest::*;
  use std::thread;
  use std::time;

  use std::fs;
  use std::path::Path;
  use super::entry;

  speculate!{
    use super::entry::ExploreInfo;
    use super::entry::enumrate_file;

    describe "enumrate_file test"{
      before{
        thread::sleep(time::Duration::from_millis(1)); // wait remove file of before tests.
        // create temp directory.
        if !Path::new("temp").exists(){
          if let Err(e) = fs::create_dir("temp") {
            println!("{}",e);
            panic!("failed create dir.");
          }
        }
      }

      after{
        if Path::new("temp").exists(){
          // remove temp directory.
          if let Err(e) = fs::remove_dir_all("temp") {
            println!("{}",e);
            panic!("failed remove dir.");
          }
        }
      }

      fn create_file(path: &str) -> fs::File{
        match fs::File::create(path){
          Ok(f) => f,
          Err(e) => {
            println!("{}", e);
            panic!("failed create file.");
          }
        }
      }

      it "should be get no file when dir is empty"{
        let info = ExploreInfo{
          path: "temp".to_string(),
        };
        let result = enumrate_file(info);
        if let Ok(path_vec) = result{
          assert_eq!(path_vec.len(), 0);
        }
        else{
          panic!("did not get ok.");
        }
      }

      it "should be get one file when dir has a file"{
        create_file("temp/temp.a");
        let info = ExploreInfo{
          path: "temp".to_string(),
        };

        let result = enumrate_file(info);

        if let Ok(path_vec) = result{
          assert_eq!(path_vec.len(), 1);
          assert_eq!(path_vec[0].path().into_os_string().into_string().unwrap(), "temp/temp.a");
        }
        else{
          panic!("did not get ok.");
        }
      }
    }

  }


}
