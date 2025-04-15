# nfd-to-nfc

맥에서 생성된 unicode nfd로 되어있는 파일 노멀라이징 해서 파일이름 변경하기

## 사용법

1. `cargo build --release --target=aarch64-apple-darwin`
2. `sudo cp target/aarch64-apple-darwin/release/nfc /usr/local/bin/`

3. 터미널 껏다키고 `nfc -h`

## 삭제하는법

`sudo rm /usr/local/bin/nfc`

빌드 해서 생긴 target/release/nfc 바이너리 파일 등록해서 사용
