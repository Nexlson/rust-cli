# set path to binary
cd ~
current_dir=$(pwd)
echo "export PATH=$current_dir/rust-cli/target/debug/executable:\$PATH" >>~/.zshrc
source ~/.zshrc
