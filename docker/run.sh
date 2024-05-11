#!/bin/bash
ROOT=$(pwd)
# 定义容器名称和镜像名称
CONTAINER_NAME="pku-compiler-dev"
IMAGE_NAME="pku-compiler-dev"

# 检查容器是否存在
container_exists=$(docker ps -a | grep $CONTAINER_NAME)

# 检查容器的运行状态
container_running=$(docker ps | grep $CONTAINER_NAME)

# 如果容器存在但未运行，则启动它
if [ -n "$container_exists" ] && [ -z "$container_running" ]; then
    docker start $CONTAINER_NAME
# 如果容器不存在，则创建并启动一个新的容器
elif [ -z "$container_exists" ]; then
    docker run --platform linux/amd64 -v $ROOT:/root/compiler -d --name $CONTAINER_NAME $IMAGE_NAME 
fi

docker exec $CONTAINER_NAME autotest -koopa -s lv1 /root/compiler
