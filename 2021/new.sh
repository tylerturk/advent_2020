if [ $# -ne 1 ]; then
    echo "Usage: $0 <path>"
    exit 1
fi
cp -av template/ $1
sed -i.bak "s/REPLACE/$1/" $1/Cargo.toml && rm -f $1/CARGO.toml.bak
