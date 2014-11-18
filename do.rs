
fn main() {

  let input_file = "given";
  let output_file = "result";

  let data = input::read(input_file);
  let result = data::process(data);
  result::write(result, output_file);
}

struct Data {
  width: u8,
  height: u8,
  data: Vec<u8>,
}


mod input {
  use std::io::File;

  pub fn read(file_name: &str) -> Vec<u8> {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();
    file.read_to_end().unwrap()
  }

}

mod data {

  pub fn process(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec!();

    // each 4-th byte is just 255 for some reason
    for rgb in data.as_slice().chunks(4) {
      result.push_all([rgb[0], rgb[1], rgb[2]]);
    }
    result
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
