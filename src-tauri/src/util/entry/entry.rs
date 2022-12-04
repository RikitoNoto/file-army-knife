
pub mod entry{

  pub struct ExploreInfo{
    pub path: String,
  }

  pub fn enumrate_file(info: &ExploreInfo){

  }
}


#[cfg(test)]
mod tests{
  extern crate speculate;

  use speculate::speculate;
  use rstest::*;

  use std::fs;
  use std::path::Path;

  use crate::util::entry::entry;

  speculate!{
    describe "enumrate_file test"{
      before{
        if !Path::new("temp").exists(){
          fs::create_dir("temp");
        }
      }

      after{
        if Path::new("temp").exists(){
          // fs::remove_dir_all("temp");
        }
      }

      it "text"{

      }
    }

  }


}
