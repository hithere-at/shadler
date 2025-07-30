#!/bin/sh

# $1: compiler
# $2: target triple
# $3: compile type
compile_shadler() {

    export RUSTFLAGS="-C linker=$(pwd)/$1"
    export CC="$(pwd)/$1"
    cargo build --release --target $2

}

target_triples=$(
cat << EOF
aarch64-linux-android
aarch64-unknown-linux-musl
armv7-linux-androideabi
arm-unknown-linux-musleabi
i686-linux-android
i686-unknown-linux-musl
x86_64-linux-android
x86_64-unknown-linux-musl
EOF
)

c_compiler_paths=$(
cat << EOF
android-ndk-r27d/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang
aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
android-ndk-r27d/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang
arm-linux-musleabi-cross/bin/arm-linux-musleabi-gcc
android-ndk-r27d/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang
i686-linux-musl-cross/bin/i686-linux-musl-gcc
android-ndk-r27d/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang
x86_64-linux-musl-cross/bin/x86_64-linux-musl-gcc
EOF
)

file_suffixes=$(
cat << EOF
aarch64-android
aarch64-linux
armv7-android
arm-linux
x86-android
x86-linux
x86_64-android
x86_64-linux
EOF
)

cross_compiler_links=$(
cat << EOF
https://dl.google.com/android/repository/android-ndk-r27d-linux.zip
https://musl.cc/aarch64-linux-musl-cross.tgz
https://musl.cc/arm-linux-musleabi-cross.tgz
https://musl.cc/i686-linux-musl-cross.tgz
https://musl.cc/x86_64-linux-musl-cross.tgz
EOF
)

mkdir compilers

find compilers/ -maxdepth 1 -type f| while read -r dir; do

    compiler=$(printf "%s" "$dir" | grep -o "\(android-ndk-r27d-linux\|aarch64-linux-musl-cross\|arm-linux-musleabi-cross\|i686-linux-musl-cross\|x86_64-linux-musl-cross\)")

    if [ -z "$compiler" ]; then

        # downloading cross compilers
        printf "installing cross compilers to %s/compilers..\n\n" "$(pwd)"

        printf "%s\n" "$cross_compiler_links" | while read -r link; do
            wget -P compilers $link

        done

        # extracting cross compilers
        find compilers/ -maxdepth 1 -type f | while read -r compiler; do

            file_type = $(printf "%s" "$compilers" | grep -o '\(zip\|tgz\)$')

            if [ $file_type = "zip" ]; then
                unzip $compiler -d compilers/

            else
                tar -xzf $compiler -C compilers/

            fi

        done

    fi

done

mkdir build

counter=1
while [ $counter -le 8 ]; do

    target=$(printf "%s\n" "$target_triples" | sed "${counter}q;d")
    compiler_path=compilers/$(printf "%s\n" "$c_compiler_paths" | sed "${counter}q;d")
    suffix=$(printf "%s\n" "$file_suffixes" | sed "${counter}q;d")

    compile_shadler $compiler_path $target

    cp target/$target/release/shadler build/shadler-$suffix

    counter=$((counter + 1))

done

