```
PC build
.cargo 파일의 config.toml
[build]
target = "aarch64-apple-ios"
주석처리 후
cargo run 
or
cargo build
```

```
iOS build
.cargo 파일의 config.toml
(만약 타겟이 실제기기가 아니라 시뮬레이터 라면 aarch64-apple-ios-sim 으로 해야한다.)
[build]
target = "aarch64-apple-ios"
주석 해제 후
sh ios_build.sh --release
이후
xcode 오픈후, 실행
```