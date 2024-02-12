use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    io::Error,
};
use zip::encode_decode_file::*;
use zip::read_encode_file::*;
use zip::save_encode_file::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let exet = file_name.split('.').collect::<Vec<&str>>();
    match exet[1] {
        "txt" => match pack_file(file_name.as_str()) {
            Ok(()) => println!("Success!"),
            Err(e) => println!("Error: {}", e),
        },
        "bin" => match unpack_file(file_name.as_str()) {
            Ok(()) => println!("Success!"),
            Err(e) => println!("Error: {}", e),
        },
        _ => println!("Unknown file type!"),
    }
}

fn pack_file(file_name: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(file_name)?;
    let contents = contents.trim().to_string();
    let chars = contents.chars().collect::<Vec<char>>();
    let mut char_counts = BTreeMap::new();
    for c in chars {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let tree = build_tree(&char_counts);
    let char_code: HashMap<char, String> = HashMap::new();
    let code = String::new();
    let encodes = haffcode(&tree.clone().root(), char_code, code);
    let enc = encode(contents, &encodes);
    let exet = file_name.split('.').collect::<Vec<&str>>();
    let file_name = format!("{}.bin", exet[0]);
    match save_dict(&file_name, &char_counts) {
        Err(e) => eprintln!("Error: {}", e),
        Ok(()) => println!("Frequencies saved successfully!"),
    }
    match save_encodes(enc.clone(), &file_name) {
        Err(e) => eprintln!("Error: {}", e),
        Ok(()) => println!("Encodes saved successfully!"),
    }
    Ok(())
}

fn unpack_file(file_name: &str) -> Result<(), Error> {
    let (freq_ch, encode) = read_file(file_name).unwrap();
    let freq = read_dict(freq_ch).unwrap();
    let tree = build_tree(&freq);
    let str_enc = read_encode(encode).unwrap();
    let decode = decode(str_enc, &tree.clone().root());
    println!("{}", decode);
    Ok(())
}