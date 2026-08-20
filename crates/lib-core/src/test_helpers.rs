use std::{
    fs::{File, OpenOptions, remove_file},
    io::{Seek, Write},
};

pub fn write_wav_header(header: &mut Vec<u8>) {
    header.extend_from_slice(
        &[
            [b'R', b'I', b'F', b'F'].as_slice(),
            &[0u8; 4],
            [b'W', b'A', b'V', b'E'].as_slice(),
        ]
        .concat(),
    );
}

pub fn write_data_chunk(header: &mut Vec<u8>) {
    header.extend_from_slice(
        &[
            [b'd', b'a', b't', b'a'].as_slice(),
            &4u32.to_le_bytes(),
            &[1, 2, 3, 4],
        ]
        .concat(),
    );
}

pub fn write_junk_chunk(header: &mut Vec<u8>) {
    header.extend_from_slice(
        &[
            [b'J', b'U', b'N', b'K'].as_slice(),
            &4u32.to_le_bytes(),
            &[1, 2, 3, 4],
        ]
        .concat(),
    );
}

pub fn write_fmt_chunk(header: &mut Vec<u8>) {
    header.extend_from_slice(
        &[
            [b'f', b'm', b't', b' '].as_slice(),
            &16u32.to_le_bytes(),
            &1u16.to_le_bytes(),
            &[0u8; 12],
            &16u16.to_le_bytes(),
        ]
        .concat(),
    );
}

pub struct TestContext {
    pub path: String,
    pub file: File,
}

pub fn setup(header: &mut Vec<u8>) -> TestContext {
    let path = format!("{}.wav", rand::random_range(0..1000));
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.write_all(header).unwrap();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();

    TestContext { path, file }
}

pub fn teardown(path: String) {
    remove_file(path).unwrap();
}
