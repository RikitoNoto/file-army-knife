
pub mod entry{

  use std::path::Path;
  pub struct ExploreInfo{
    pub path: String,
  }

  pub fn enumrate_file(info: &ExploreInfo) -> Vec<&Path>{
    Vec::<&Path>::new()
  }
}


#[cfg(test)]
mod tests{
  extern crate speculate;

  use speculate::speculate;
  use rstest::*;

  use std::fs;
  use std::path::Path;
  use super::entry;

  speculate!{
    use super::entry::ExploreInfo;
    use super::entry::enumrate_file;

    describe "enumrate_file test"{
      before{
        // create temp directory.
        if !Path::new("temp").exists(){
          if let Err(_) = fs::create_dir("temp") {
            assert!(false);
          }
        }
      }

      after{
        if Path::new("temp").exists(){
          // remove temp directory.
          if let Err(_) = fs::remove_dir_all("temp") {
            assert!(false);
          }
        }
      }

      it "should be get no file when dir is empty"{
        let info = ExploreInfo{
          path: "temp".to_string(),
        };
        let path_vec = enumrate_file(&info);
        assert_eq!(path_vec.len(), 0);
      }

      it "should be get one file when dir has a file"{
        fs::File::create("temp/temp.a");
        let info = ExploreInfo{
          path: "temp".to_string(),
        };

        let path_vec = enumrate_file(&info);
        assert_eq!(path_vec.len(), 1);
        assert_eq!(path_vec[0].display().to_string(), "temp/temp.a");
      }
    }

  }


}
