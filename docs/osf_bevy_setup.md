# OpenSeeFace + Bevy 開発環境セットアップ

依存を増やさずにネイティブ OpenSeeFace (OSF) バイナリを JSON/UDP で送信し、Bevy アプリで受信するまでの流れをまとめます。Python は使いません。

## 前提
- Rust (stable), cargo
- CMake, a C++17 コンパイラ (gcc/clang/Visual Studio)
- OS (Linux/macOS/Windows)。以下のコマンド例は Linux 向けです。

### Bevy 実行に必要になりやすいライブラリ (Linux)
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

## 1) OpenSeeFace ネイティブバイナリの用意

### A. リリース済みバイナリを使う (推奨)
1. リリース配布されている OSF バイナリを取得（例: `OpenSeeFace` 実行ファイル）。  
2. 任意の作業フォルダに置き、実行権限を付与:
   ```bash
   chmod +x OpenSeeFace
   ```

### B. ソースからビルドする場合
1. 取得:
   ```bash
   git clone https://github.com/emilianavt/OpenSeeFace.git
   cd OpenSeeFace
   ```
2. 依存の用意: ONNX Runtime と OpenCV の開発ヘッダとライブラリをインストール。  
   - 例 (Ubuntu): `sudo apt-get install -y libopencv-dev`  
   - ONNX Runtime は公式バイナリをダウンロードして `onnxruntime` ディレクトリに配置するか、CMake で外部取得する形をとります。
3. ビルド (CMake):
   ```bash
   mkdir build && cd build
   cmake .. -DCMAKE_BUILD_TYPE=Release
   cmake --build . --config Release
   ```
4. 出力された `OpenSeeFace` (または `OpenSeeFace.exe`) を実行に使います。

※ 環境ごとに ONNX/OpenCV のパス指定が必要な場合があります。ビルドが難しい場合はリリースバイナリを使う方が手早いです。

## 2) OSF の実行 (JSON/UDP 送出)
Bevy 側は `127.0.0.1:11573` で JSON を待ち受ける構成になっています。OSF を次のように起動してください:
```bash
./OpenSeeFace --onnx --tracking 1 --udp 11573 --output_format json --landmarks 68 --faces 1
```
- `--udp 11573` : localhost:11573 に送信
- `--output_format json` : JSON 形式
- `--faces 1` : 1 顔のみ

JSON 例 (先頭の顔だけ使用):
```json
{
  "faces": [
    {
      "rotation": [0.01, -0.02, 0.03],
      "translation": [0.0, 0.1, 0.5]
    }
  ]
}
```

## 3) Bevy アプリの実行
このリポジトリは OSF の JSON/UDP を受信し、キューブとステータスを更新します。
```bash
cargo run
```
- OSF データが届くと、キューブの位置・回転と左上テキストが更新されます。
- データ未取得時はキューブがゆっくり回転し、ステータスは「Waiting...」表示になります。

## トラブルシュート
- **パース失敗/無反応**: OSF の JSON フィールド名や回転単位/順序が異なる場合は `src/main.rs` の `OsfFace` 構造体やオイラー角の適用順を調整してください。
- **ポート競合**: 11573 が使用中なら `src/main.rs` 内 `OSF_ADDR` と OSF 起動時の `--udp` を合わせて変更します。
- **座標系が逆**: `apply_face_pose` 内で軸反転や単位スケールをまとめて適用してください。
- **依存ライブラリ不足 (ビルド時)**: OpenCV/ONNX のヘッダ/ライブラリを認識できるように `CMAKE_PREFIX_PATH` などを指定します。迅速に試す場合はリリースバイナリ利用を検討してください。
