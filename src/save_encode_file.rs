use std::{collections::BTreeMap, fs::{File, OpenOptions}, io::{BufWriter, Error, Write}};

pub fn save_dict(file_name: &str, dict: &BTreeMap<char, usize>) -> Result<(), std::io::Error> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&dict.len().to_le_bytes()[..1]).unwrap();    
    for (key, value) in dict {
        let k = vec![0_u8; 4];
        let mut kk = key.to_string().as_bytes().to_vec();
        kk.extend(k);
        writer.write_all(&kk[..3]).unwrap();
        let tmp = &value.to_le_bytes()[..3];
        writer.write_all(tmp).unwrap();
    }
    Ok(())
}

pub fn save_encodes(encode: String, file_name: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_name)?;
    let iter = encode.as_bytes().chunks_exact(8);
    let mut bytes = iter.clone().map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(),2)
                    .unwrap()).collect::<Vec<u8>>();
    let remein = iter.remainder();
    if !remein.is_empty() {
        let ost = u8::from_str_radix(std::str::from_utf8(remein).unwrap(),2).unwrap();
        let ost = ost << (8 - remein.len() as u8);
        bytes.push(ost);
        bytes.push((8 - remein.len() + 48) as u8);
    } else {
        bytes.push(48);
    }
    file.write_all(&bytes)?;
    Ok(())
}