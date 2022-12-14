

pub mod line_counter {
  extern crate regex;
  use regex::Regex;




  pub fn slice_between_string<'a>(target_str: &'a str, start_str: &str, end_str: &str, use_regex_start_end: bool) -> Vec<&'a str>{
    let mut vec: Vec<&str> = Vec::new();
    if use_regex_start_end {
      search_slice_with_regex(target_str, start_str, end_str, &mut vec);
    }
    else {
      seach_slice(target_str, start_str, end_str, &mut vec);
    }

    vec
  }

  fn search_slice_with_regex<'a>(target_str: &'a str, start_str: &str, end_str: &str, vec: & mut Vec<&'a str>){
    let re = Regex::new(&format!(r"(?s)(?:{}\n?(.*?){}(.*))", start_str, end_str)).unwrap();
    let cap = re.captures(target_str);

    match cap {
      None => {},
      Some(v) => {
        vec.push(v.get(1).map_or("", |s| s.as_str()));
        search_slice_with_regex(v.get(2).map_or("", |s| s.as_str()), start_str, end_str, vec);
      },
    }

  }

  fn seach_slice<'a>(target_str: &'a str, start_str: &str, end_str: &str, vec: & mut Vec<&'a str>){
    let slice_str: &str;
    let start_point = target_str.find(start_str);
    let mut start_len = 0;

    if target_str.find(&(format!("{}\n", start_str))).is_some(){
      start_len = start_str.len() + 1;
    }
    else if start_point.is_some(){
      start_len = start_str.len();
    }

    let end_point = target_str.find(end_str);

    if start_point.is_some() && end_point.is_some(){
      if start_point.unwrap() < end_point.unwrap(){
        slice_str = &(target_str[start_point.unwrap() + start_len .. end_point.unwrap()]);
        vec.push(slice_str);
        seach_slice(&target_str[end_point.unwrap() + end_str.len()..], start_str, end_str, vec)
      }
    }

  }

}



#[cfg(test)]
mod tests {
  //! tests

  use crate::line_counter::line_counter::line_counter;

  /// should be return the empty vector when call [slice_between_string] with the args.
  /// # Arguments
  /// * `_target_str` - ""
  /// * `_start_str` - ""
  /// * `_end_str` - ""
  /// # Expect
  /// * `vector length` - 0
  #[test]
  fn should_be_return_empty_vec_when_target_arg_is_all_empty() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("", "", "", false);
    assert_eq!(slice_string.len(), 0);
  }

  /// should be return the empty vector when call [slice_between_string] with the args.
  /// # Arguments
  /// * `_target_str` - ""
  /// * `_start_str` - "start"
  /// * `_end_str` - "end"
  /// # Expect
  /// * `vector length` - 0
  #[test]
  fn should_be_return_empty_vec_when_target_arg_is_empty() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("", "start", "end", false);
    assert_eq!(slice_string.len(), 0);
  }

  /// should be get a space when call [slice_between_string] with the args.
  /// # Arguments
  /// * `_target_str` - "start end"
  /// * `_start_str` - "start"
  /// * `_end_str` - "end"
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - [" "]
  #[test]
  fn should_be_return_vec_of_one_content_when_target_args_are_start_end_and_start_and_end() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start end", "start", "end", false);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], " ");
  }

  /// should be get a char 'a' when call [slice_between_string] with the args.
  /// # Arguments
  /// * `_target_str` - "startaend"
  /// * `_start_str` - "start"
  /// * `_end_str` - "end"
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - ["a"]
  #[test]
  fn should_be_return_vec_of_one_content_a_when_target_args_are_startaend_and_start_and_end() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("startaend", "start", "end", false);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], "a");
  }

  /// should be get chars from call [slice_between_string] when arg include with line feed.
  /// # Arguments
  /// * `_target_str` - "start\na\nend"
  /// * `_start_str` - "start"
  /// * `_end_str` - "end"
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - ["a\n"]
  #[test]
  fn should_be_get_chars_when_args_include_line_feed() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start\na\nend", "start", "end", false);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], "a\n");
  }

  /// should be get chars from call [slice_between_string] when arg include with some line feeds.
  /// # Arguments
  /// * `_target_str` - "start\n\na\nend"
  /// * `_start_str` - "start"
  /// * `_end_str` - "end"
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - ["\na\n"]
  #[test]
  fn should_be_get_chars_when_args_include_some_line_feeds() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start\n\na\nend", "start", "end", false);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], "\na\n");
  }

  /// should be use regex to args of start and end.
  /// # Arguments
  /// * `_target_str` - "start\na\nend"
  /// * `_start_str` - "s.*t\n"
  /// * `_end_str` - "e.d"
  /// * `_use_regex_start_end` - true
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - ["a\n"]
  #[test]
  fn should_be_use_regex_to_args_of_start_and_end() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start\na\nend", r"s.*t\n", r"e.d", true);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], "a\n");
  }

  /// should be able to disable regex to args of start and end.
  /// # Arguments
  /// * `_target_str` - "s.*t\nt\na\nende.*d"
  /// * `_start_str` - "s.*t\n"
  /// * `_end_str` - "e.*d"
  /// * `_use_regex_start_end` - false
  /// # Expect
  /// * `vector length` - 1
  /// * `vector content` - ["t\na\nend"]
  #[test]
  fn should_be_able_to_disable_regex_to_args_of_start_and_end() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("s.*t\nt\na\nende.*d", "s.*t\n", "e.*d", false);
    assert_eq!(slice_string.len(), 1);
    assert_eq!(slice_string[0], "t\na\nend");
  }

  /// should be able to slice some chars.
  /// # Arguments
  /// * `_target_str` - "start\na\nendstart\nb\nend"
  /// * `_start_str` - "start\n"
  /// * `_end_str` - "end"
  /// * `_use_regex_start_end` - false
  /// # Expect
  /// * `vector length` - 2
  /// * `vector content` - ["a", "b"]
  #[test]
  fn should_be_able_to_slice_some_chars() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start\na\nendstart\nb\nend", "start\n", "end", false);
    assert_eq!(slice_string.len(), 2);
    assert_eq!(slice_string[0], "a\n");
    assert_eq!(slice_string[1], "b\n");
  }

  /// should be able to slice some chars with regex.
  /// # Arguments
  /// * `_target_str` - "start\na\nendstart\nb\nend"
  /// * `_start_str` - "s.*t\n"
  /// * `_end_str` - "e.d"
  /// * `_use_regex_start_end` - true
  /// # Expect
  /// * `vector length` - 2
  /// * `vector content` - ["a", "b"]
  #[test]
  fn should_be_able_to_slice_some_chars_with_regex() {
    let slice_string: Vec<&str> = line_counter::slice_between_string("start\na\nendstart\nb\nend", "s.*?t\n", "e.d", true);
    assert_eq!(slice_string.len(), 2);
    assert_eq!(slice_string[0], "a\n");
    assert_eq!(slice_string[1], "b\n");
  }
}
