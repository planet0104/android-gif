:: set ANDROID_NDK="C:/Android/SDK/ndk/21.1.6352462"
:: set ANDROID_TOOLCHAIN=F:\ndk-standalone-16-arm\bin
::设置要临时加入到path环境变量中的路径
::set PATH=%PATH%;%ANDROID_TOOLCHAIN%

cargo build --target armv7-linux-androideabi --release
cargo build --target aarch64-linux-android --release
cargo build --target i686-linux-android --release
cargo build --target x86_64-linux-android --release

copy target\armv7-linux-androideabi\release\libandroid_gif.so ..\..\jniLibs\armeabi-v7a\libandroid_gif.so
copy target\aarch64-linux-android\release\libandroid_gif.so ..\..\jniLibs\arm64-v8a\libandroid_gif.so
copy target\x86_64-linux-android\release\libandroid_gif.so ..\..\jniLibs\x86_64\libandroid_gif.so
copy target\i686-linux-android\release\libandroid_gif.so ..\..\jniLibs\x86\libandroid_gif.so