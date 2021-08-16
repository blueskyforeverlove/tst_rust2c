# tst_rust2c</br>
Rust复杂结构与C进行交互</br>
复杂结构的构造及销毁通过Rust的方法，该方法暴露给C;C调用Rust的函数时传参通过void*指针;</br></br>
**编译Rust库**</br>
cargo build --release</br></br>
**编译C代码(需要先编译Rust库)**</br>
cd src/bin/ && gcc main.c -ltst_rust2c -L ../../target/release/ -Wl,-rpath,./ -o ../../target/release/main -ggdb -O0