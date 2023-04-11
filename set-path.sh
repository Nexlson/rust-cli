# set path to binary
current_dir=$(pwd)
echo "export PATH=$current_dir/executable:\$PATH" >>~/.zshrc
source ~/.zshrc
