#[macro_use]
extern crate trackable;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// cf. https://github.com/frugalos/cannyls/wiki/Storage-Format
fn header_size(lusf_file_name: String) -> u16 {
    let mut buffer: Vec<u8> = vec![0; 4 * 2]; // 32bit * 2
    let mut file = track_try_unwrap!(track_any_err!(File::open(lusf_file_name)));
    track_try_unwrap!(track_any_err!(file.read(&mut buffer)));

    let mut two_bytes: [u8; 2] = Default::default();
    two_bytes.copy_from_slice(&buffer[4..6]);

    u16::from_be_bytes(two_bytes)
}

// cf. https://github.com/frugalos/cannyls/wiki/Storage-Format
fn journal_size(lusf_file_name: String) -> u64 {
    let mut buffer: Vec<u8> = vec![0; 4 * 9]; // 32bit * 9
    let mut file = track_try_unwrap!(track_any_err!(File::open(lusf_file_name)));
    track_try_unwrap!(track_any_err!(file.read(&mut buffer)));

    let mut eight_bytes: [u8; 8] = Default::default();
    eight_bytes.copy_from_slice(&buffer[28..36]);

    u64::from_be_bytes(eight_bytes)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("[Usage] cargo run input.lusf output.lusf");
        return;
    }

    let output_file_name: String = args[2].clone();
    if Path::new(&output_file_name).exists() {
        println!("{} does already exist", output_file_name);
        return;
    }

    let lusf_file_name: String = args[1].clone();
    println!(
        "extract the header and journal regions from {} and create & write to {}",
        lusf_file_name, output_file_name
    );

    let total_size: usize;

    total_size = header_size(lusf_file_name.clone()) as usize
        + journal_size(lusf_file_name.clone()) as usize;

    // ヘッダ領域+ジャーナル領域を読み込む
    let mut buffer = vec![0; total_size];
    let mut file = track_try_unwrap!(track_any_err!(File::open(lusf_file_name)));
    track_try_unwrap!(track_any_err!(file.read(&mut buffer)));

    // 読み込んだ部分をファイル作成して書き出す
    let mut f = File::create(output_file_name.clone()).expect("Unable to create file");
    let written_size: usize = track_try_unwrap!(track_any_err!(f.write(&buffer)));

    // 全て書き出せていなかった場合は失敗として通知
    if written_size == total_size {
        println!("[Success]")
    } else {
        println!(
            "[Failed] we only wrote {}-bytes from {}-bytes",
            written_size, total_size
        );
    }
}
