FROM ubuntu

WORKDIR /app

RUN apt update && apt install -qy git build-essential
RUN apt install -qy libreadline-dev

RUN git clone https://github.com/berkeley-abc/abc

WORKDIR /app/abc

RUN make

CMD /bin/bash
