#!/bin/bash
set -xeuo pipefail

cargo test --features c-headers -- generate_headers

TARGETS=(
  'aarch64-apple-ios'
  'x86_64-apple-darwin'
  'aarch64-apple-darwin'
  'x86_64-apple-ios'
  'aarch64-apple-ios-sim'
)
for target in "${TARGETS[@]}"
do
  rustup target add $target
  cargo build --release --target $target
done

#TARGETS=(
#  'aarch64-apple-ios-macabi'
#  'x86_64-apple-ios-macabi'
#)
#for target in "${TARGETS[@]}"
#do
#  cargo +nightly build -Z build-std --release --target $target
#done

lipo -create \
  target/x86_64-apple-darwin/release/libgranne_c.a \
  target/aarch64-apple-darwin/release/libgranne_c.a \
  -output libgranne_c_macos.a
lipo -create \
  target/x86_64-apple-ios/release/libgranne_c.a \
  target/aarch64-apple-ios-sim/release/libgranne_c.a \
  -output libgranne_c_iossimulator.a
#lipo -create \
#  target/aarch64-apple-ios-macabi/release/libgranne_c.a \
#  target/x86_64-apple-ios-macabi/release/libgranne_c.a \
#  -output libgranne_c_maccatalyst.a
rm -rf Granne.xcframework
xcodebuild -create-xcframework \
  -library ./libgranne_c_macos.a \
  -headers ./include/ \
  -library ./libgranne_c_iossimulator.a \
  -headers ./include/ \
  -library ./target/aarch64-apple-ios/release/libgranne_c.a \
  -headers ./include/ \
  -output Granne.xcframework
#  -library ./libgranne_c_maccatalyst.a \
#  -headers ./include/ \
