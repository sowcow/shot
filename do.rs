
fn main() {

  let input_file = "given";
  let output_file = "result";

  let data = input::read(input_file);
  let result = data::process(data);
  result::write(result, output_file);
}


mod input {
  use std::io::File;

  pub fn read(file: &str) -> Vec<u8> {
    vec![1u8,2,3]
  }
}

mod data {
  //static RGB_RECORD_SIZE: uint = 4;
  // 4-th is always = 255

  pub fn process(data: Vec<u8>) -> Vec<u8> {
    data
  }
}

mod result {
  use std::io::File;

  pub fn write(data: Vec<u8>, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = try!( File::create(&path) );
  }
}
