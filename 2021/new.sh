if [ $# -ne 1 ]; then
    echo "Usage: $0 <path>"
    exit 1
fi
rsync -aP template/ $1
sed -i "s/REPLACE/$1/" $1/Cargo.toml