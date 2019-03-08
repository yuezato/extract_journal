extern crate cannyls;
#[macro_use]
extern crate trackable;

use cannyls::nvm::FileNvm;
use cannyls::storage::{Storage, StorageBuilder, StorageHeader};

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
    let original_header: StorageHeader;

    // オリジナルのヘッダ部分を取り出し、ファイル先頭から何バイト取り出せばよいか算出
    {
        let file_nvm: FileNvm =
            track_try_unwrap!(track_any_err!(FileNvm::open(lusf_file_name.clone())));
        let storage: Storage<FileNvm> =
            track_try_unwrap!(track_any_err!(StorageBuilder::new().open(file_nvm)));

        original_header = storage.header().clone();

        let header_size = original_header.region_size();
        let journal_region_size = original_header.journal_region_size;
        total_size = (header_size + journal_region_size) as usize;

        // drop file descriptors related to `lusf_file_name`.
    }

    // 読み込みと書き出しを行う
    {
        // ヘッダ領域+ジャーナル領域を読み込む
        let mut buffer = vec![0; total_size];
        let mut file = track_try_unwrap!(track_any_err!(File::open(lusf_file_name)));
        track_try_unwrap!(track_any_err!(file.read(&mut buffer)));

        // 読み込んだ部分をファイル作成して書き出す
        let mut f = File::create(output_file_name.clone()).expect("Unable to create file");
        let written_size: usize = track_try_unwrap!(track_any_err!(f.write(&buffer)));

        // 全て書き出せていなかった場合は失敗として通知
        if written_size != total_size {
            println!(
                "[Failed] we only wrote {}-bytes from {}-bytes",
                written_size, total_size
            );
            return;
        }
    }

    // 取り出した部分とオリジナルの比較を一部行う（簡易検査）
    {
        let file_nvm: FileNvm = track_try_unwrap!(track_any_err!(FileNvm::open(output_file_name)));
        let storage: Storage<FileNvm> =
            track_try_unwrap!(track_any_err!(StorageBuilder::new().open(file_nvm)));

        let copied_header = storage.header().clone();

        if original_header.major_version == copied_header.major_version
            && original_header.minor_version == copied_header.minor_version
            && original_header.instance_uuid == copied_header.instance_uuid
        {
            println!("[Maybe Success]");
            return;
        } else {
            println!("[Failed]");

            dbg!(original_header);

            dbg!(copied_header);
        }
    }
}
