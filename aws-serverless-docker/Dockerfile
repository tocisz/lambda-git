FROM ubuntu
RUN apt-get update &&\
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl xz-utils git less groff unzip &&\
    rm -rf /var/lib/apt/lists/*

WORKDIR /root

# Serverless
RUN curl -o- -L https://slss.io/install | bash

# AWS cli
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" &&\
  unzip awscliv2.zip &&\
  ./aws/install &&\
  rm -rf aws awscliv2.zip

# docker client
ENV DOCKER_VERSION=19.03.9
RUN curl https://download.docker.com/linux/static/stable/x86_64/docker-$DOCKER_VERSION.tgz -o docker-$DOCKER_VERSION.tgz &&\
  tar xzf docker-$DOCKER_VERSION.tgz && install docker/docker /usr/local/bin &&\
  rm -rf docker docker-$DOCKER_VERSION.tgz

# nodejs
ENV NODE_VERSION=v14.11.0
RUN curl https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-linux-x64.tar.xz -o node-$NODE_VERSION-linux-x64.tar.xz &&\
    tar xJf node-$NODE_VERSION-linux-x64.tar.xz && cd node-$NODE_VERSION-linux-x64 && cp -ra * /usr/local &&\
    cd /root && rm -rf node-$NODE_VERSION-linux-x64*

CMD /bin/bash