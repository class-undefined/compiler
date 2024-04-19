ROOT=$(pwd)

docker run -it --rm -v $ROOT:/root/compiler maxxing/compiler-dev bash
