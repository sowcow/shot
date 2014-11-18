
// todo into one call
fn main() {

  let input_file = "given";
  let output_file = "result";

  let data = input::read(input_file);
  let result = data::process(data);
  result::write(result, output_file);
}


mod types {
  pub struct ImageData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
  }
}


mod input {
  use std::io::File;
  use types::ImageData;

  pub fn read(file_name: &str) -> ImageData {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();

    let width = file.read_le_i32().unwrap();
    let height = file.read_le_i32().unwrap();
    let _unknown = file.read_le_i32().unwrap();
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

    // each 4-th byte is just `255` for some reason
    for rgb in data.data.as_slice().chunks(4) {
      result.data.push_all([rgb[0], rgb[1], rgb[2]]);
    }
    result
  }
}

mod result {
  use std::io::File;
  use types::ImageData;

  // todo: into ImageData struct impl!
  // .ppm format
  pub fn write(image: ImageData, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = File::create(&path).unwrap();

    let width = image.width;
    let height = image.height;
    let header = format!("P6 {} {} 255\n", width, height);
    file.write(header.as_bytes()).unwrap();
    file.write(image.data.as_slice()).unwrap();
  }
}
