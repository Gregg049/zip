use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error, Read};
use std::str::Utf8Error;

pub fn read_file(file_name: &str) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let mut file = File::open(file_name)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let tmp = *contents.first().unwrap() as usize;
    let len_freq = contents[1..=tmp * 6].to_vec();
    let encode = contents[1 + tmp..].to_vec();
    Ok((len_freq, encode))
}

pub fn read_dict(bytes: Vec<u8>) -> Result<BTreeMap<char, usize>, std::io::Error> {
    let mut dict = BTreeMap::new();
    for chunk in bytes.chunks_exact(6) {
        let key = String::from_utf8_lossy(&chunk[..3])
            .trim_end_matches('\0')
            .to_string();
        let value = u32::from_le_bytes([chunk[3], chunk[4], chunk[5], 0]);
        dict.insert(key.chars().next().unwrap(), value as usize);
    }
    Ok(dict)
}

pub fn read_encode(mut bytes: Vec<u8>) -> Result<String, Utf8Error> {
    let binding = bytes.pop();
    let ost = std::str::from_utf8(binding.as_slice())?;
    let code = bytes
        .iter()
        .fold("".to_string(), |acc, c| acc + &format!("{:08b}", c));
    let code_haff = &code[..code.len() - ost.parse::<usize>().unwrap()];
    Ok(code_haff.to_string())
}
