apt install libsoapysdr-dev libclang-dev llvm-dev pkg-config

# Choose the appropriate drivers for your hardware:
apt install soapysdr-module-rtlsdr soapysdr-module-hackrf soapysdr-module-uhd soapysdr-module-lms7

git clone git@github.com:kevinmehall/rust-soapysdr.git@v0.4.5