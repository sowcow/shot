
fn main() {

  let input_file = "given";
  let output_file = "result";

  let data = input::read(input_file);
  let result = data::process(data);
  result::write(result, output_file);
}


mod types {
  pub struct ImageData {
    pub width: u8,
    pub height: u8,
    pub data: Vec<u8>,
  }
}


mod input {
  use std::io::File;
  use types::ImageData;

  pub fn read(file_name: &str) -> ImageData {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();
    //
    // todo: read header data as well
    let width = 1;
    let height = 1;
    let data = file.read_to_end().unwrap();

    ImageData { width: width, height: height, data: data }
  }

}

mod data {
  use types::ImageData;

  pub fn process(data: ImageData) -> ImageData {
    let mut result = ImageData {
      width: data.width,
      height: data.height,
      data: vec!()
    };

    // each 4-th byte is just 255 for some reason
    for rgb in data.data.as_slice().chunks(4) {
      result.data.push_all([rgb[0], rgb[1], rgb[2]]);
    }
    result
  }
}

mod result {
  use std::io::File;
  use types::ImageData;

  pub fn write(image: ImageData, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = File::create(&path).unwrap();
    file.write(image.data.as_slice()).unwrap();
  }
}
