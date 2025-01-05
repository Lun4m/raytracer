#!/bin/bash

target=$1

if [[ $target == "windows" ]]; then
    cargo build --release -p raytracer-gpu --target x86_64-pc-windows-gnu
    ./target/x86_64-pc-windows-gnu/release/raytracer-gpu.exe
elif [[ $target == "linux" ]]; then
    cargo build --release -p raytracer-gpu --target x86_64-unknown-linux-gnu
    ./target/x86_64-unknown-linux-gnu/release/raytracer-gpu
else
    echo "USAGE: ./gpu.sh <compilation-target>"
    echo "The target can either be \"windows\" or \"linux\""
fi
