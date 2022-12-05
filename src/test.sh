# swap contents of two files
# Usage: swap file1 file2

# check if two arguments are passed
if [ $# -ne 2 ]; then
    echo "Usage: swap file1 file2"
    exit 1
fi

# check if files exist
if [ ! -f $1 ]; then
    echo "File $1 does not exist"
    exit 1
fi

if [ ! -f $2 ]; then
    echo "File $2 does not exist"
    exit 1
fi

# swap contents of files
mv $1 $1.tmp
mv $2 $1
mv $1.tmp $2
