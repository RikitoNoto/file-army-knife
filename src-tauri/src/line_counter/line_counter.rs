

pub mod line_counter {
  extern crate regex;
  use regex::Regex;

  pub fn slice_between_string<'a>(_target_str: &'a str, _start_str: &str, _end_str: &str, _use_regex_start_end: bool) -> Vec<&'a str>{
    if _use_regex_start_end {
      slice_between_string_with_regex(_target_str, _start_str, _end_str)
    }
    else {
      slice_between_string_with_plain_text(_target_str, _start_str, _end_str)
    }
  }

  fn slice_between_string_with_regex<'a>(_target_str: &'a str, _start_str: &str, _end_str: &str) -> Vec<&'a str>{
    let _re = Regex::new(&format!(r"(?s){}(?:\n)?(.*){}", _start_str, _end_str)).unwrap();
    let _cap = _re.captures(_target_str);
    let mut _vec = Vec::new();

    match _cap {
      None => {},
      Some(v) => {
        _vec.push(v.get(1).map_or("", |s| s.as_str()));
      },
    }

    _vec
  }

  fn slice_between_string_with_plain_text<'a>(_target_str: &'a str, _start_str: &str, _end_str: &str) -> Vec<&'a str>{
    let mut _vec: Vec<&str> = Vec::new();
    let _start_point = _target_str.find(_start_str);
    let mut _start_len = 0;

    if _target_str.find(&(format!("{}\n", _start_str))).is_some(){
      _start_len = _start_str.len() + 1;
    }
    else if _start_point.is_some(){
      _start_len = _start_str.len();
    }

    let _end_point =_target_str.find(_end_str);

    if _start_point.is_some() && _end_point.is_some(){
      if _start_point.unwrap() < _end_point.unwrap(){
        _vec.push(&(_target_str[_start_point.unwrap() + _start_len .. _end_point.unwrap()]));
      }
    }

    _vec
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
  #[ignore]
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
}
