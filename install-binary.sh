# remove previous binary
rm -r target/debug/executable/

# build project
cargo build

# move binary to another folder
cd target/debug
mkdir executable
mv jarvis executable/
