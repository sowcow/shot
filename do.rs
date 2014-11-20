extern crate time; // deprecated one ...

// ...
fn diff() {
  use time::get_time;

  let now = get_time();
  println!("{}", now);
}

fn main() {
  // shot task...
  // funny that name of modules represent dependencies
  //diff();
  files::get_raw_screen("raw_screen");
  //diff();
  files::raw_to_ppm("raw_screen", "screen.ppm");
  //diff();
  files::ppm_to_bmp("screen.ppm", "screen.bmp");
  //diff();
}

mod files {

  pub fn raw_to_ppm(input_file: &str, output_file: &str) {
    use input;
    use data;
    use result;

    let data = input::read(input_file);
    let result = data::process(data);
    result::write(result, output_file);
  }

  pub fn get_raw_screen(file_name: &str) {
    use std::io::Command;
    //let output = Command::new("adb").arg("pull").arg("/dev/graphics/fb0").arg(file_name).output().unwrap();

    // not using specific file names and dont cleaning up
    let output = Command::new("adb").args(["shell","screencap","/sdcard/raw"]).output().unwrap();
    if !output.status.success() { fail!("get_raw fail1"); }

    let output = Command::new("adb").args(["pull","/sdcard/raw",file_name]).output().unwrap();
    if !output.status.success() { fail!("get_raw fail1"); }
  }

  pub fn ppm_to_bmp(ppm: &str, bmp: &str) {
    use std::io::Command;
    let output = Command::new("convert").args([ppm,"-rotate", "-90",bmp]).output().unwrap();
    if !output.status.success() { fail!("ppm2bmp fail"); }
  }
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
