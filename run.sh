!#/bin/bash

#Kills all previous docker containers

#containers=$(docker ps -q) &&
#docker kill $containers &&

cargo build --release &&
docker build -t kill-rs . && 
docker run -it --rm kill-rs /bin/bash 
