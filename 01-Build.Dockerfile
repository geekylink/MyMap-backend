FROM alpine

MAINTAINER James Danielson

RUN apk update
RUN apk upgrade
RUN apk add rustup build-base yarn

RUN adduser -D rusty
RUN su rusty -c 'rustup-init -q -y'
RUN su rusty -c 'mkdir $HOME/www'

ADD ./ /home/rusty/www/
RUN chown -R rusty:rusty /home/rusty/www/
RUN echo "Building Server..."
RUN rm -f /home/rusty/www/www/build
RUN su rusty -c 'source $HOME/.cargo/env && cd /home/rusty/www/ && make clean && make'
#RUN su rusty -c 'source $HOME/.cargo/env && cd /home/rusty/www/ && cargo build --release'

#RUN su rusty -c 'cd /home/rusty/www/www/ && rm -f build && yarn install && yarn build'

