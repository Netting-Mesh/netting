# Reason for docker image is because DNS was failing when I tried to run `apt update` within the init container
FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y iptables

