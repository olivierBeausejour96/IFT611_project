# IFT611_project
High Frequency Trading in Rust

# How to use:  
Install the Rust toolchain: https://www.rust-lang.org/tools/install  
Build the binaries ``cargo build --release``  
Start the server in one shell ``target/release/server data.csv -p 8080 --period 100000``  
Start the client in another ``target/release/client http://127.0.0.1:8080``  

# Running the benchmarks:
Simply run ``cargo bench``. The report will be in ``target/criterion/report/index.html``  

For you convenience, pre-built binaries and benchmark reports are placed in ``dist/``  
