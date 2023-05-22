/*
AES implementation in rust
2023
Ricardo Hernandez Lopez
*/
use std::convert::TryInto;
use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;
//
pub struct Config {
    pub mode: String,
    pub key: String,
    pub input_file_path: String,
    pub output_file_path: String,
}
//
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            info();
            return Err("not enough arguments");
        }
        let mode = args[1].clone();
        let key  = args[2].clone();
        let input_file_path = args[3].clone();
        let output_file_path = args[4].clone();
        Ok(Config { mode,key, input_file_path, output_file_path })
    }
}
//
fn open_key_file(config: &Config) -> String {
    let key = fs::read_to_string(&config.key)
        .expect("Should have been able to read the key file");
    println!("key length: {}",key.len());
    key
}
//
fn read_input_file(config: &Config) -> (Vec<u8>, usize) {
    let bytes = fs::read(&config.input_file_path).unwrap();
    let mut vector_contents:Vec<u8>= vec![];
    let mut size = 0;
    for (index,one_byte) in bytes.chunks_exact(1).enumerate() {
        vector_contents.push(one_byte[0]);
        size = index;
    }
    println!("Input file {} with size {}",&config.input_file_path,size);
    (vector_contents,size)
}
//
fn write_output_file(config: &Config, contents: Vec<u8>) {
    let mut file = File::create(&config.output_file_path).unwrap();
    let mut size = 0;
    for (index,value) in contents.iter().enumerate() {
       file.write_all(&value.to_le_bytes()).unwrap();
       size = index;
    }
    println!("Output file {} with size {}",&config.output_file_path,size);
}
//
fn string2array(key: String) -> [u8;32]{
  let mut result:[u8;32]= [0; 32];
  let mut test: &mut[u8] = &mut result;
  test.write(key.as_bytes()).unwrap(); 
  result
}
//
fn state2data_block(state:[[u8;4];4]) ->[u8;16] {
  let mut result:[u8;16]=[0;16];
  for (k, element) in state.iter().flat_map(|r| r.iter()).enumerate() {
        result[k]=*element;
 }
  result
}
//
fn info(){
  println!("AES Encryption Algorithm Version 0.1.0\n");
  println!("aes <options> <KEY file> <input file> <outputfile>");
  println!("<options> e: encryption, d: decryption");
}
// Galois Field (256) Multiplication of two Bytes
fn gmul(a:u8,b:u8) -> u8{
    let by2:[u8;256] =[
0x00,0x02,0x04,0x06,0x08,0x0a,0x0c,0x0e,0x10,0x12,0x14,0x16,0x18,0x1a,0x1c,0x1e,
0x20,0x22,0x24,0x26,0x28,0x2a,0x2c,0x2e,0x30,0x32,0x34,0x36,0x38,0x3a,0x3c,0x3e,
0x40,0x42,0x44,0x46,0x48,0x4a,0x4c,0x4e,0x50,0x52,0x54,0x56,0x58,0x5a,0x5c,0x5e,
0x60,0x62,0x64,0x66,0x68,0x6a,0x6c,0x6e,0x70,0x72,0x74,0x76,0x78,0x7a,0x7c,0x7e,	
0x80,0x82,0x84,0x86,0x88,0x8a,0x8c,0x8e,0x90,0x92,0x94,0x96,0x98,0x9a,0x9c,0x9e,
0xa0,0xa2,0xa4,0xa6,0xa8,0xaa,0xac,0xae,0xb0,0xb2,0xb4,0xb6,0xb8,0xba,0xbc,0xbe,
0xc0,0xc2,0xc4,0xc6,0xc8,0xca,0xcc,0xce,0xd0,0xd2,0xd4,0xd6,0xd8,0xda,0xdc,0xde,
0xe0,0xe2,0xe4,0xe6,0xe8,0xea,0xec,0xee,0xf0,0xf2,0xf4,0xf6,0xf8,0xfa,0xfc,0xfe,
0x1b,0x19,0x1f,0x1d,0x13,0x11,0x17,0x15,0x0b,0x09,0x0f,0x0d,0x03,0x01,0x07,0x05,
0x3b,0x39,0x3f,0x3d,0x33,0x31,0x37,0x35,0x2b,0x29,0x2f,0x2d,0x23,0x21,0x27,0x25,
0x5b,0x59,0x5f,0x5d,0x53,0x51,0x57,0x55,0x4b,0x49,0x4f,0x4d,0x43,0x41,0x47,0x45,
0x7b,0x79,0x7f,0x7d,0x73,0x71,0x77,0x75,0x6b,0x69,0x6f,0x6d,0x63,0x61,0x67,0x65,
0x9b,0x99,0x9f,0x9d,0x93,0x91,0x97,0x95,0x8b,0x89,0x8f,0x8d,0x83,0x81,0x87,0x85,
0xbb,0xb9,0xbf,0xbd,0xb3,0xb1,0xb7,0xb5,0xab,0xa9,0xaf,0xad,0xa3,0xa1,0xa7,0xa5,
0xdb,0xd9,0xdf,0xdd,0xd3,0xd1,0xd7,0xd5,0xcb,0xc9,0xcf,0xcd,0xc3,0xc1,0xc7,0xc5,
0xfb,0xf9,0xff,0xfd,0xf3,0xf1,0xf7,0xf5,0xeb,0xe9,0xef,0xed,0xe3,0xe1,0xe7,0xe5];
    let by3:[u8;256] =[
0x00,0x03,0x06,0x05,0x0c,0x0f,0x0a,0x09,0x18,0x1b,0x1e,0x1d,0x14,0x17,0x12,0x11,
0x30,0x33,0x36,0x35,0x3c,0x3f,0x3a,0x39,0x28,0x2b,0x2e,0x2d,0x24,0x27,0x22,0x21,
0x60,0x63,0x66,0x65,0x6c,0x6f,0x6a,0x69,0x78,0x7b,0x7e,0x7d,0x74,0x77,0x72,0x71,
0x50,0x53,0x56,0x55,0x5c,0x5f,0x5a,0x59,0x48,0x4b,0x4e,0x4d,0x44,0x47,0x42,0x41,
0xc0,0xc3,0xc6,0xc5,0xcc,0xcf,0xca,0xc9,0xd8,0xdb,0xde,0xdd,0xd4,0xd7,0xd2,0xd1,
0xf0,0xf3,0xf6,0xf5,0xfc,0xff,0xfa,0xf9,0xe8,0xeb,0xee,0xed,0xe4,0xe7,0xe2,0xe1,
0xa0,0xa3,0xa6,0xa5,0xac,0xaf,0xaa,0xa9,0xb8,0xbb,0xbe,0xbd,0xb4,0xb7,0xb2,0xb1,
0x90,0x93,0x96,0x95,0x9c,0x9f,0x9a,0x99,0x88,0x8b,0x8e,0x8d,0x84,0x87,0x82,0x81,	
0x9b,0x98,0x9d,0x9e,0x97,0x94,0x91,0x92,0x83,0x80,0x85,0x86,0x8f,0x8c,0x89,0x8a,
0xab,0xa8,0xad,0xae,0xa7,0xa4,0xa1,0xa2,0xb3,0xb0,0xb5,0xb6,0xbf,0xbc,0xb9,0xba,
0xfb,0xf8,0xfd,0xfe,0xf7,0xf4,0xf1,0xf2,0xe3,0xe0,0xe5,0xe6,0xef,0xec,0xe9,0xea,	
0xcb,0xc8,0xcd,0xce,0xc7,0xc4,0xc1,0xc2,0xd3,0xd0,0xd5,0xd6,0xdf,0xdc,0xd9,0xda,	
0x5b,0x58,0x5d,0x5e,0x57,0x54,0x51,0x52,0x43,0x40,0x45,0x46,0x4f,0x4c,0x49,0x4a,
0x6b,0x68,0x6d,0x6e,0x67,0x64,0x61,0x62,0x73,0x70,0x75,0x76,0x7f,0x7c,0x79,0x7a,	
0x3b,0x38,0x3d,0x3e,0x37,0x34,0x31,0x32,0x23,0x20,0x25,0x26,0x2f,0x2c,0x29,0x2a,
0x0b,0x08,0x0d,0x0e,0x07,0x04,0x01,0x02,0x13,0x10,0x15,0x16,0x1f,0x1c,0x19,0x1a];
    let by9:[u8;256] =[
0x00,0x09,0x12,0x1b,0x24,0x2d,0x36,0x3f,0x48,0x41,0x5a,0x53,0x6c,0x65,0x7e,0x77,
0x90,0x99,0x82,0x8b,0xb4,0xbd,0xa6,0xaf,0xd8,0xd1,0xca,0xc3,0xfc,0xf5,0xee,0xe7,
0x3b,0x32,0x29,0x20,0x1f,0x16,0x0d,0x04,0x73,0x7a,0x61,0x68,0x57,0x5e,0x45,0x4c,
0xab,0xa2,0xb9,0xb0,0x8f,0x86,0x9d,0x94,0xe3,0xea,0xf1,0xf8,0xc7,0xce,0xd5,0xdc,
0x76,0x7f,0x64,0x6d,0x52,0x5b,0x40,0x49,0x3e,0x37,0x2c,0x25,0x1a,0x13,0x08,0x01,
0xe6,0xef,0xf4,0xfd,0xc2,0xcb,0xd0,0xd9,0xae,0xa7,0xbc,0xb5,0x8a,0x83,0x98,0x91,
0x4d,0x44,0x5f,0x56,0x69,0x60,0x7b,0x72,0x05,0x0c,0x17,0x1e,0x21,0x28,0x33,0x3a,
0xdd,0xd4,0xcf,0xc6,0xf9,0xf0,0xeb,0xe2,0x95,0x9c,0x87,0x8e,0xb1,0xb8,0xa3,0xaa,	
0xec,0xe5,0xfe,0xf7,0xc8,0xc1,0xda,0xd3,0xa4,0xad,0xb6,0xbf,0x80,0x89,0x92,0x9b,	
0x7c,0x75,0x6e,0x67,0x58,0x51,0x4a,0x43,0x34,0x3d,0x26,0x2f,0x10,0x19,0x02,0x0b,
0xd7,0xde,0xc5,0xcc,0xf3,0xfa,0xe1,0xe8,0x9f,0x96,0x8d,0x84,0xbb,0xb2,0xa9,0xa0,
0x47,0x4e,0x55,0x5c,0x63,0x6a,0x71,0x78,0x0f,0x06,0x1d,0x14,0x2b,0x22,0x39,0x30,
0x9a,0x93,0x88,0x81,0xbe,0xb7,0xac,0xa5,0xd2,0xdb,0xc0,0xc9,0xf6,0xff,0xe4,0xed,
0x0a,0x03,0x18,0x11,0x2e,0x27,0x3c,0x35,0x42,0x4b,0x50,0x59,0x66,0x6f,0x74,0x7d,	
0xa1,0xa8,0xb3,0xba,0x85,0x8c,0x97,0x9e,0xe9,0xe0,0xfb,0xf2,0xcd,0xc4,0xdf,0xd6,
0x31,0x38,0x23,0x2a,0x15,0x1c,0x07,0x0e,0x79,0x70,0x6b,0x62,0x5d,0x54,0x4f,0x46];
    let by11:[u8;256] =[
0x00,0x0b,0x16,0x1d,0x2c,0x27,0x3a,0x31,0x58,0x53,0x4e,0x45,0x74,0x7f,0x62,0x69,
0xb0,0xbb,0xa6,0xad,0x9c,0x97,0x8a,0x81,0xe8,0xe3,0xfe,0xf5,0xc4,0xcf,0xd2,0xd9,
0x7b,0x70,0x6d,0x66,0x57,0x5c,0x41,0x4a,0x23,0x28,0x35,0x3e,0x0f,0x04,0x19,0x12,
0xcb,0xc0,0xdd,0xd6,0xe7,0xec,0xf1,0xfa,0x93,0x98,0x85,0x8e,0xbf,0xb4,0xa9,0xa2,
0xf6,0xfd,0xe0,0xeb,0xda,0xd1,0xcc,0xc7,0xae,0xa5,0xb8,0xb3,0x82,0x89,0x94,0x9f,
0x46,0x4d,0x50,0x5b,0x6a,0x61,0x7c,0x77,0x1e,0x15,0x08,0x03,0x32,0x39,0x24,0x2f,
0x8d,0x86,0x9b,0x90,0xa1,0xaa,0xb7,0xbc,0xd5,0xde,0xc3,0xc8,0xf9,0xf2,0xef,0xe4,
0x3d,0x36,0x2b,0x20,0x11,0x1a,0x07,0x0c,0x65,0x6e,0x73,0x78,0x49,0x42,0x5f,0x54,
0xf7,0xfc,0xe1,0xea,0xdb,0xd0,0xcd,0xc6,0xaf,0xa4,0xb9,0xb2,0x83,0x88,0x95,0x9e,
0x47,0x4c,0x51,0x5a,0x6b,0x60,0x7d,0x76,0x1f,0x14,0x09,0x02,0x33,0x38,0x25,0x2e,
0x8c,0x87,0x9a,0x91,0xa0,0xab,0xb6,0xbd,0xd4,0xdf,0xc2,0xc9,0xf8,0xf3,0xee,0xe5,
0x3c,0x37,0x2a,0x21,0x10,0x1b,0x06,0x0d,0x64,0x6f,0x72,0x79,0x48,0x43,0x5e,0x55,
0x01,0x0a,0x17,0x1c,0x2d,0x26,0x3b,0x30,0x59,0x52,0x4f,0x44,0x75,0x7e,0x63,0x68,
0xb1,0xba,0xa7,0xac,0x9d,0x96,0x8b,0x80,0xe9,0xe2,0xff,0xf4,0xc5,0xce,0xd3,0xd8,
0x7a,0x71,0x6c,0x67,0x56,0x5d,0x40,0x4b,0x22,0x29,0x34,0x3f,0x0e,0x05,0x18,0x13,
0xca,0xc1,0xdc,0xd7,0xe6,0xed,0xf0,0xfb,0x92,0x99,0x84,0x8f,0xbe,0xb5,0xa8,0xa3];
    let by13:[u8;256] =[
0x00,0x0d,0x1a,0x17,0x34,0x39,0x2e,0x23,0x68,0x65,0x72,0x7f,0x5c,0x51,0x46,0x4b,
0xd0,0xdd,0xca,0xc7,0xe4,0xe9,0xfe,0xf3,0xb8,0xb5,0xa2,0xaf,0x8c,0x81,0x96,0x9b,
0xbb,0xb6,0xa1,0xac,0x8f,0x82,0x95,0x98,0xd3,0xde,0xc9,0xc4,0xe7,0xea,0xfd,0xf0,
0x6b,0x66,0x71,0x7c,0x5f,0x52,0x45,0x48,0x03,0x0e,0x19,0x14,0x37,0x3a,0x2d,0x20,
0x6d,0x60,0x77,0x7a,0x59,0x54,0x43,0x4e,0x05,0x08,0x1f,0x12,0x31,0x3c,0x2b,0x26,
0xbd,0xb0,0xa7,0xaa,0x89,0x84,0x93,0x9e,0xd5,0xd8,0xcf,0xc2,0xe1,0xec,0xfb,0xf6,
0xd6,0xdb,0xcc,0xc1,0xe2,0xef,0xf8,0xf5,0xbe,0xb3,0xa4,0xa9,0x8a,0x87,0x90,0x9d,
0x06,0x0b,0x1c,0x11,0x32,0x3f,0x28,0x25,0x6e,0x63,0x74,0x79,0x5a,0x57,0x40,0x4d,
0xda,0xd7,0xc0,0xcd,0xee,0xe3,0xf4,0xf9,0xb2,0xbf,0xa8,0xa5,0x86,0x8b,0x9c,0x91,
0x0a,0x07,0x10,0x1d,0x3e,0x33,0x24,0x29,0x62,0x6f,0x78,0x75,0x56,0x5b,0x4c,0x41,
0x61,0x6c,0x7b,0x76,0x55,0x58,0x4f,0x42,0x09,0x04,0x13,0x1e,0x3d,0x30,0x27,0x2a,
0xb1,0xbc,0xab,0xa6,0x85,0x88,0x9f,0x92,0xd9,0xd4,0xc3,0xce,0xed,0xe0,0xf7,0xfa,
0xb7,0xba,0xad,0xa0,0x83,0x8e,0x99,0x94,0xdf,0xd2,0xc5,0xc8,0xeb,0xe6,0xf1,0xfc,
0x67,0x6a,0x7d,0x70,0x53,0x5e,0x49,0x44,0x0f,0x02,0x15,0x18,0x3b,0x36,0x21,0x2c,
0x0c,0x01,0x16,0x1b,0x38,0x35,0x22,0x2f,0x64,0x69,0x7e,0x73,0x50,0x5d,0x4a,0x47,
0xdc,0xd1,0xc6,0xcb,0xe8,0xe5,0xf2,0xff,0xb4,0xb9,0xae,0xa3,0x80,0x8d,0x9a,0x97];
   let by14:[u8;256] =[
0x00,0x0e,0x1c,0x12,0x38,0x36,0x24,0x2a,0x70,0x7e,0x6c,0x62,0x48,0x46,0x54,0x5a,
0xe0,0xee,0xfc,0xf2,0xd8,0xd6,0xc4,0xca,0x90,0x9e,0x8c,0x82,0xa8,0xa6,0xb4,0xba,
0xdb,0xd5,0xc7,0xc9,0xe3,0xed,0xff,0xf1,0xab,0xa5,0xb7,0xb9,0x93,0x9d,0x8f,0x81,
0x3b,0x35,0x27,0x29,0x03,0x0d,0x1f,0x11,0x4b,0x45,0x57,0x59,0x73,0x7d,0x6f,0x61,
0xad,0xa3,0xb1,0xbf,0x95,0x9b,0x89,0x87,0xdd,0xd3,0xc1,0xcf,0xe5,0xeb,0xf9,0xf7,
0x4d,0x43,0x51,0x5f,0x75,0x7b,0x69,0x67,0x3d,0x33,0x21,0x2f,0x05,0x0b,0x19,0x17,
0x76,0x78,0x6a,0x64,0x4e,0x40,0x52,0x5c,0x06,0x08,0x1a,0x14,0x3e,0x30,0x22,0x2c,
0x96,0x98,0x8a,0x84,0xae,0xa0,0xb2,0xbc,0xe6,0xe8,0xfa,0xf4,0xde,0xd0,0xc2,0xcc,
0x41,0x4f,0x5d,0x53,0x79,0x77,0x65,0x6b,0x31,0x3f,0x2d,0x23,0x09,0x07,0x15,0x1b,
0xa1,0xaf,0xbd,0xb3,0x99,0x97,0x85,0x8b,0xd1,0xdf,0xcd,0xc3,0xe9,0xe7,0xf5,0xfb,
0x9a,0x94,0x86,0x88,0xa2,0xac,0xbe,0xb0,0xea,0xe4,0xf6,0xf8,0xd2,0xdc,0xce,0xc0,
0x7a,0x74,0x66,0x68,0x42,0x4c,0x5e,0x50,0x0a,0x04,0x16,0x18,0x32,0x3c,0x2e,0x20,
0xec,0xe2,0xf0,0xfe,0xd4,0xda,0xc8,0xc6,0x9c,0x92,0x80,0x8e,0xa4,0xaa,0xb8,0xb6,
0x0c,0x02,0x10,0x1e,0x34,0x3a,0x28,0x26,0x7c,0x72,0x60,0x6e,0x44,0x4a,0x58,0x56,
0x37,0x39,0x2b,0x25,0x0f,0x01,0x13,0x1d,0x47,0x49,0x5b,0x55,0x7f,0x71,0x63,0x6d,
0xd7,0xd9,0xcb,0xc5,0xef,0xe1,0xf3,0xfd,0xa7,0xa9,0xbb,0xb5,0x9f,0x91,0x83,0x8d];
    if a==2 { return by2[b as usize]; } 
    if a==3 { return by3[b as usize]; }
    if a==9 { return by9[b as usize]; }
    if a==11 { return by11[b as usize]; }
    if a==13 { return by13[b as usize]; }
    if a==14 { return by14[b as usize]; } 
    else {
      0 
    }
}
//
fn mix_columns(s:[[u8;4];4]) -> [[u8; 4]; 4]{
    let mut result:[[u8; 4]; 4]=[[0;4];4];
    for c in 0..4 {
       result[c][0]= gmul(2,s[c][0])^gmul(3,s[c][1])^s[c][2]^s[c][3];
       result[c][1]= s[c][0]^ gmul(2,s[c][1])^gmul(3,s[c][2])^s[c][3];
       result[c][2]= s[c][0]^ s[c][1]^ gmul(2,s[c][2])^gmul(3,s[c][3]);
       result[c][3]= gmul(3,s[c][0])^s[c][1]^s[c][2] ^gmul(2,s[c][3]);
    }
    result
}
//
fn inv_mix_columns(s:[[u8;4];4]) -> [[u8; 4]; 4]{
    let mut result:[[u8; 4]; 4]=[[0;4];4];
    for c in 0..4 {
       result[c][0]= gmul(0x0e, s[c][0])^gmul(0x0b, s[c][1])^gmul(0x0d,s[c][2])^gmul(0x09,s[c][3]);
       result[c][1]= gmul(0x09, s[c][0])^gmul(0x0e, s[c][1])^gmul(0x0b,s[c][2])^gmul(0x0d,s[c][3]);
       result[c][2]= gmul(0x0d, s[c][0])^gmul(0x09, s[c][1])^gmul(0x0e,s[c][2])^gmul(0x0b,s[c][3]);
       result[c][3]= gmul(0x0b, s[c][0])^gmul(0x0d, s[c][1])^gmul(0x09,s[c][2])^gmul(0x0e,s[c][3]);
    }
    result
}
//
fn add_round_key(state:[[u8; 4];4] , w:[[[u8; 4];4];15],keycount:usize) -> [[u8;4];4] {
     let mut result:[[u8;4];4]=[[0;4];4];
     for c in 0..4 {
       for r in 0..4 {
         result[c][r]=state[c][r]^w[keycount][c][r];
       }
     }
     result
}
//
fn sub_word(word: [u8; 4]) -> [u8; 4]{
  let s:[u8;256]=[  
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
  ];
  let mut result:[u8; 4]=[0;4];
  for col in 0..4 {
     result[col]=s[word[col] as usize];
  }
  result
}
//
fn sub_bytes(state:[[u8; 4]; 4]) -> [[u8; 4]; 4] {
  let mut result:[[u8; 4]; 4]=[[0;4];4];
  for col in 0..4 {
     result[col]=sub_word(state[col]);
  }
  result
}
//
fn inv_sub_bytes(state:[[u8; 4]; 4]) -> [[u8; 4]; 4] {
  let inv_s:[u8;256]=[
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D
 ];

  let mut result:[[u8; 4]; 4]=[[0;4];4];
  for row in 0..4 {
    for col in 0..4 {
       result[col][row]=inv_s[ state[col][row] as usize ];
    }
  }
  result
}
//
fn inv_shift_rows(state:[[u8;4];4]) -> [[u8; 4]; 4]{
    let mut result:[[u8; 4]; 4]=[[0;4];4];
    let s_col:[usize; 16]=[0,1,2,3,3,0,1,2,2,3,0,1,1,2,3,0];
    let row:[usize; 16]  =[0,0,0,0, 1,1,1,1, 2,2,2,2, 3,3,3,3];
    for item in 0..16 {
       result[item%4][row[item]]= state[s_col[item]][row[item]] ;
    }
    result
}
//
fn shift_rows(state:[[u8;4];4]) -> [[u8; 4]; 4]{
    let mut result:[[u8; 4]; 4]=[[0;4];4];
    let s_col:[usize; 16]=[0,1,2,3, 1,2,3,0, 2,3,0,1, 3,0,1,2];
    let row:[usize; 16]  =[0,0,0,0, 1,1,1,1, 2,2,2,2, 3,3,3,3];
    for item in 0..16 {
       result[item%4][row[item]]= state[s_col[item]][row[item]] ;
    }
    result
}
//
fn key_expansion(key:[u8; 32],dec:bool) -> [[[u8; 4] ;4]; 15]{ 
  let rcon:[u8;256]=[
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 
    0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 
    0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 
    0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 
    0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 
    0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 
    0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 
    0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 
    0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 
    0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 
    0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 
    0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 
    0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 
    0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 
    0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 
    0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d
  ];

  let mut w:[[u8; 4]; 60] = [[0; 4]; 60];
  for i in 0..8 {
    w[i]=[ key[4*i], key[4*i+1], key[4*i+2], key[4*i+3]];
  }
  for i in 8..60 {
     let mut tmp = [ w[i-1][0], w[i-1][1], w[i-1][2], w[i-1][3] ];
     if i% 8 == 0 {
       tmp = sub_word(  rot_word(tmp) );
       tmp[0]^=rcon[i/8];
     } else if i%8==4 {
       tmp = sub_word( tmp );
     }

    for (x, item) in tmp.iter().enumerate(){
      w[i][x]=w[i-8][x] ^ item;
    }
  }
  let mut keys:[[[u8; 4]; 4]; 15] = [[[0; 4] ;4]; 15];
  for x in 0..15{
    keys[x]=[w[4*x],w[4*x+1],w[4*x+2],w[4*x+3]];
  }
  if !dec {
   keys
  } else {
    // skip keys 0 and 14
    for x in 1..14{
      keys[x]=inv_mix_columns([w[4*x],w[4*x+1],w[4*x+2],w[4*x+3]]);
    } 
    keys
  }
} 
//
fn rot_word(w:[u8;4]) -> [u8; 4]{
  let mut result:[u8;4]=[0;4];
  result[3]=w[0];
  result[0]=w[1];
  result[1]=w[2];
  result[2]=w[3];
  result
}
//
fn create_state(data:[u8;16]) ->[[u8;4];4]{
   let mut state:[[u8; 4]; 4] = [[0; 4]; 4];
   let mut counter = 0;
   for row in 0..4 {
     for col in 0..4 {
       state[row][col]=data[counter];
       counter+=1;
     }
   }
   state   
}
//
fn aes_encrypt(mut input:Vec<u8>, z:[u8;32],size:usize) -> Vec<u8>{
   let mut result:Vec<u8> = vec![];
   let mut block:[u8;16];
   let padding:usize = size%16;
   let mut w:usize = size+padding+16;
   for _i in 0..padding+16{
       let x:u8 = 0x80;
       input.push(x);
   }
   //println!("key:{z:?}"); 
   let keys=key_expansion(z,false);
   //println!("key_expansion:{keys:?}");
   let mut g = 0;
   loop {
     if w<16 { return result; }
     //println!("Round: 0");
     block = input[g..(g+16)].try_into().unwrap(); // block of 16 bytes = 128 bits
     //println!("input:{block:?}");
     let mut state = create_state(block); 
     //println!("start:{state:?}");
     //println!("keys:{:?}",keys[0]);     

     state=add_round_key(state,keys,0);
     //println!("add_round_key:{state:?}");
     for i in 1..14 {
       //println!("Round: {i}");
       state = sub_bytes(state);
       //println!("sub_bytes:{state:?}");
       state = shift_rows(state);
       //println!("shift_rows:{state:?}");
       state = mix_columns(state);
       //println!("mix_columns:{state:?}");
       state = add_round_key(state,keys,i);
       //println!("add_round_key:{state:?}");
       //println!("keys:{:?}",keys[i]);
     }
     //println!("Round: 14");
     state = sub_bytes(state);
     //println!("sub_bytes:{state:?}");
     state = shift_rows(state);
     //println!("shift_rows:{state:?}");
     state = add_round_key(state,keys,14);
     //println!("add_round_key:{state:?}");
     //println!("keys:{:?}",keys[14]);
     let last = state2data_block(state);
     //println!("last:{last:?}");
     result.extend(last.to_vec().iter().copied());
     w-=16;
     g+=16;
  }
}
//
fn aes_decrypt(mut input:Vec<u8>, z:[u8;32],size:usize) -> Vec<u8>{
  let mut result:Vec<u8> = vec![];
  let mut block:[u8;16];
  let padding:usize = size%16;
  let mut w:usize= size+padding;
  for _i in 0..padding{
       let x:u8 = 0x80;
       input.push(x);
  }
  //println!("key:{z:?}");
  let keys = key_expansion(z,true);
  //println!("key_expansion:{keys:?}");
  let mut g = 0;
  loop {
    if w<16 { return result; }
    //println!("Round: 14");
    block = input[g..(g+16)].try_into().unwrap(); // block of 16 bytes = 128 bits
    //println!("block:{block:?}");
    let mut state = create_state(block);
    //println!("start:{state:?}");
    state = add_round_key(state,keys,14);
    //println!("add_round_key:{state:?}");
    //println!("key:{:?}",keys[14]);
    for i in (1..14).rev() {
      //println!("Round: {i}");
      //println!("start:{state:?}");
      state = inv_sub_bytes(state);
      //println!("inv_sub_bytes:{state:?}");
      state = inv_shift_rows(state);
      //println!("inv_shift_rows:{state:?}");
      state = inv_mix_columns(state);
      //println!("inv_mix_columns:{state:?}");
      state = add_round_key(state,keys,i);
      //println!("add_round_key:{state:?}");
      //println!("key:{:?}",keys[i]);
    } 
    //println!("Round: 0");
    //println!("start:{state:?}");
    state = inv_sub_bytes(state);
    //println!("inv_sub_bytes:{state:?}");
    state = inv_shift_rows(state);
    //println!("inv_shift_rows:{state:?}");
    state = add_round_key(state,keys,0);
    //println!("add_round_key:{state:?}");
    //println!("key:{:?}",keys[0]);
    let last = state2data_block(state);
    //println!("last:{last:?}");
    result.extend(last.to_vec().iter().copied());
    w-=16;
    g+=16;
  }
}
//
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
  let key = open_key_file(&config);
  let (vector_contents,size) = read_input_file(&config);
  println!("size:{size}");
  match config.mode.as_str() {
        // Encrypt
        "e" => {
            println!("Encrypt!");
            let encryption_keys = string2array(key);
            let output = aes_encrypt(vector_contents,encryption_keys,size);
            write_output_file(&config, output);
        },
        // Decrypt
        "d" => {
            println!("Decrypt!");
            let decryption_keys = string2array(key);
            let output = aes_decrypt(vector_contents,decryption_keys,size);
            write_output_file(&config, output);
        },
        _ => info(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_state() {
       let block:[u8;16]=[0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3];
       let state = create_state(block);
       let expected:[[u8;4];4]=[[0,1,2,3],[0,1,2,3],[0,1,2,3],[0,1,2,3]];
       assert_eq!(state,expected);
    }
    #[test]
    fn test_string2array() {
       let key = "01234567890123456789012345678901".to_string();
       let expected:[u8;32]=[48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49];
       let encryption_keys = string2array(key);
       assert_eq!(encryption_keys,expected);
    }
   #[test]
   fn test_state2data_block() {
       let state:[[u8;4];4]=[[0,1,2,3],[0,1,2,3],[0,1,2,3],[0,1,2,3]];
       let expected:[u8;16]=[0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3];
       let last = state2data_block(state);
       assert_eq!(last,expected);
    }
   #[test]
   fn test_rot_word() {
       let word:[u8;4]=[0,1,2,3];
       let expected:[u8;4]=[1,2,3,0];
       let result = rot_word(word);
       assert_eq!(result,expected);
    }
   #[test]
   fn test_mix_unmix_columns() {
       // https://en.wikipedia.org/wiki/Rijndael_MixColumns#Test_vectors_for_MixColumn()
       let state:[[u8;4];4]=[[0xdb, 0x13, 0x53, 0x45],[0xf2, 0x0a, 0x22, 0x5c],[0x01, 0x01, 0x01, 0x01],[0xc6,0xc6,0xc6,0xc6]];
       let mix = mix_columns(state);
       let expected = [[142, 77, 161, 188],[159, 220, 88, 157],[1,1,1,1],[198,198,198,198]];
       let unmix = inv_mix_columns(mix);
       assert_eq!(mix,expected);
       assert_eq!(unmix,state);
    }
   #[test]
   fn test_shift_rows() {
       let state:[[u8;4];4]=[[0,1,2,3],[0,1,2,3],[0,1,2,3],[0,1,2,3]];
       let mix = shift_rows(state);
       let unmix = inv_shift_rows(mix);
       assert_eq!(state,unmix);
    }
   #[test]
   fn test_key_expansion() {
       let key:[u8;32]= [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
       let keys = [[[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]], [[16, 17, 18, 19], [20, 21, 22, 23], [24, 25, 26, 27], [28, 29, 30, 31]], [[165, 115, 194, 159], [161, 118, 196, 152], [169, 127, 206, 147], [165, 114, 192, 156]], [[22, 81, 168, 205], [2, 68, 190, 218], [26, 93, 164, 193], [6, 64, 186, 222]], [[174, 135, 223, 240], [15, 241, 27, 104], [166, 142, 213, 251], [3, 252, 21, 103]], [[109, 225, 241, 72], [111, 165, 79, 146], [117, 248, 235, 83], [115, 184, 81, 141]], [[198, 86, 130, 127], [201, 167, 153, 23], [111, 41, 76, 236], [108, 213, 89, 139]], [[61, 226, 58, 117], [82, 71, 117, 231], [39, 191, 158, 180], [84, 7, 207, 57]], [[11, 220, 144, 95], [194, 123, 9, 72], [173, 82, 69, 164], [193, 135, 28, 47]], [[69, 245, 166, 96], [23, 178, 211, 135], [48, 13, 77, 51], [100, 10, 130, 10]], [[124, 207, 247, 28], [190, 180, 254, 84], [19, 230, 187, 240], [210, 97, 167, 223]], [[240, 26, 250, 254], [231, 168, 41, 121], [215, 165, 100, 74], [179, 175, 230, 64]], [[37, 65, 254, 113], [155, 245, 0, 37], [136, 19, 187, 213], [90, 114, 28, 10]], [[78, 90, 102, 153], [169, 242, 79, 224], [126, 87, 43, 170], [205, 248, 205, 234]], [[36, 252, 121, 204], [191, 9, 121, 233], [55, 26, 194, 60], [109, 104, 222, 54]]];
       let expanded = key_expansion(key,false);
       println!("{:?}",expanded);
       assert_eq!(expanded,keys);
       let keys_d =[[[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]], [[26, 31, 24, 29], [30, 27, 28, 25], [18, 23, 16, 21], [22, 19, 20, 17]], [[42, 40, 64, 201], [36, 35, 76, 192], [38, 36, 76, 197], [32, 39, 72, 196]], [[127, 215, 133, 15], [97, 204, 153, 22], [115, 219, 137, 3], [101, 200, 157, 18]], [[21, 198, 104, 189], [49, 229, 36, 125], [23, 193, 104, 184], [55, 230, 32, 124]], [[174, 213, 88, 22], [207, 25, 193, 0], [188, 194, 72, 3], [217, 10, 213, 17]], [[222, 105, 64, 154], [239, 140, 100, 231], [248, 77, 12, 95], [207, 171, 44, 35]], [[248, 95, 196, 243], [55, 70, 5, 243], [139, 132, 77, 240], [82, 142, 152, 225]], [[60, 166, 151, 21], [211, 42, 243, 242], [43, 103, 255, 173], [228, 204, 211, 142]], [[116, 218, 123, 163], [67, 156, 126, 80], [200, 24, 51, 160], [154, 150, 171, 65]], [[181, 112, 142, 19], [102, 90, 125, 225], [77, 61, 130, 76], [169, 241, 81, 194]], [[200, 163, 5, 128], [139, 63, 123, 208], [67, 39, 72, 112], [217, 177, 227, 49]], [[94, 22, 72, 235], [56, 76, 53, 10], [117, 113, 183, 70], [220, 128, 230, 132]], [[52, 241, 209, 255], [191, 206, 170, 47], [252, 233, 226, 95], [37, 88, 1, 110]], [[36, 252, 121, 204], [191, 9, 121, 233], [55, 26, 194, 60], [109, 104, 222, 54]]];
      let expanded_d = key_expansion(key,true);
      println!("{:?}",expanded_d);
       assert_eq!(expanded_d,keys_d);
    }
}
