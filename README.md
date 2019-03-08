LUSFファイルからヘッダとジャーナルを切り出すためのツール

## 使い方
`test.lusf`ファイルのヘッダとジャーナルを切り出したいとする。

この時は、
```
cargo run test.lusf output.lusf
```
とする。ただし、`output.lusf`はこのコマンドで新規に作成したいので、存在してはならない。

## 例
```
$ kanils Create --storage test.lusf --capacity 10000000
passed data region size = 10000000
---------------
actual data region size = 9999872
actual journal region size = 391168
actual journal region size ratio = 0.03764474008376448

$ kanils Put --storage test.lusf --key 1 --value 🦀
[new] put key=1, value=🦀

$ kanils Header --storage test.lusf
header =>
  major version = 1
  minor version = 1
  block size = 512
  uuid = 6341c0b9-98e1-4e10-b6b8-1b97dac6b8e2
  journal region size = 391168
    journal header size = 512
    journal record size = 390656
  data region size = 9999872
  storage header size => 512
  storage total size = 10391552

$ cargo run test.lusf output.lusf
   Compiling separate_journal v0.1.0 (/Users/yuuya_uezato/Rust/separate_journal)
    Finished dev [unoptimized + debuginfo] target(s) in 1.99s
     Running `target/debug/separate_journal test.lusf output.lusf`
extract the header and journal regions from test.lusf and create & write to output.lusf
[Maybe Success]

$ kanils Header --storage output.lusf
header =>
  major version = 1
  minor version = 1
  block size = 512
  uuid = 6341c0b9-98e1-4e10-b6b8-1b97dac6b8e2
  journal region size = 391168
    journal header size = 512
    journal record size = 390656
  data region size = 9999872
  storage header size => 512
  storage total size = 10391552
```
