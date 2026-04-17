FROM ubuntu:latest
LABEL authors="singl"

ENTRYPOINT ["top", "-b"]