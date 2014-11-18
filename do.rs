
fn main() {

  let input_file = "given";
  let output_file = "result";

  let data = input::read(input_file);
  let result = data::process(data);
  result::write(result, output_file);
}


mod input {
  //use std::io::File;

  pub fn read(_file: &str) -> Vec<u8> {
    vec![1u8,2,3]
  }
}

mod data {

  pub fn process(data: Vec<u8>) -> Vec<u8> {
    let record_size = 4; // 4-th byte is kinda not used

    for _rgb in data.as_slice().chunks(record_size) {
    }
    data
  }
}

mod result {
  use std::io::File;

  pub fn write(data: Vec<u8>, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = File::create(&path).unwrap();
    file.write(data.as_slice()).unwrap();
  }
}
