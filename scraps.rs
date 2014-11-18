
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


  // preparing file apth
  //let input_path = Path::new(INPUT_FILE);
  //let input_display = path.display();

  //// opening file
  //let mut file = match File::open(&path) {
  //  Err(why) => fail!("open({}) > {}", display, why.desc),
  //  Ok(file) => file,
  //};



  //let mut file = File::create(&Path::new("message.txt"));
  //file.write(b"hello, file!\n");



  
  //let input = open_input(INPUT_FILE);
  //skip_header(&mut input); // it has width and height btw
  //let data = read_all(&mut input);


  // println!("processing...");
  //for rgb in data.as_slice().chunks(RGB_RECORD_SIZE) {
  //  append_rgb(rgb);
  //  //append_rgb(output, rgb);
  //  //println!("{} is a number!", x);
  //  //if x >= 255 {
  //  //  println!("{} is a number!", x);
  //  //}
  //}

//
//fn read_input(file_name: &'static str) -> Vec<u8> {
//}
//
//fn open_input(file_name: str) -> File {
//  //use std::io::File;
//
//  // preparing file apth
//  let path = Path::new(INPUT_FILE);
//  let display = path.display();
//
//  // opening file
//  let mut file = match File::open(&path) {
//    Err(why) => fail!("open({}) > {}", display, why.desc),
//    Ok(file) => file,
//  };
//  file
//}
//
//fn append_rgb(rgb: &[u8]) {
//  println!("{}", rgb);
//}
//
//fn read_all(file: &mut File) -> Vec<u8> {
//  file.read_to_end().unwrap()
//}
//
//fn skip_header(file: &mut File) {
//  file.read_le_i32().unwrap();
//  file.read_le_i32().unwrap();
//  file.read_le_i32().unwrap();
//}

//fn read_byte(file: &mut File) -> u8 {
//  file.read_byte().unwrap()
//}
//
//fn read_rgb(file: &mut File) {
//  let r = read_byte(file);
//  let g = read_byte(file);
//  let b = read_byte(file);
//  let _ = read_byte(file);
//  println!("got {} {} {}", r, g, b);
//}


  //match file.read_to_string() {
  //  Err(why) => fail!("{} > {}", display, why.desc),
  //  Ok(string) => print!("{}:\n{}", display, string),
  //}

//fn main() {
//  println!("Starting, input file: {}", FILE);
//
//  // Create a path to the desired file
//  let path = Path::new(FILE);
//  let display = path.display();
//
//  // Open the path in read-only mode, returns `IoResult<File>`
//  let mut file = match File::open(&path) {
//        // The `desc` field of `IoError` is a string that describes the error
//        Err(why) => fail!("couldn't open {}: {}", display, why.desc),
//        Ok(file) => file,
//    };
//
//    // Read the file contents into a string, returns `IoResult<String>`
//    match file.read_to_string() {
//        Err(why) => fail!("couldn't read {}: {}", display, why.desc),
//        Ok(string) => print!("{} contains:\n{}", display, string),
//    }
//
//    // `file` goes out of scope, and the "hello.txt" file gets closed
//}
