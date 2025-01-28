# LAUNCH challenge

FROM ubuntu@sha256:e3f92abc0967a6c19d0dfa2d55838833e947b9d74edbcb0113e48535ad4be12a AS builder

# Challenge metadata and artifacts go here. Only root has access
RUN mkdir /challenge && \
    chmod 700 /challenge

COPY server.tar.gz /app/
WORKDIR /app

RUN tar czvf /challenge/artifacts.tar.gz server.tar.gz && \
    echo "{\"flag\":\"picoCTF{p4ch1nk0_f146_0n3_e947b9d7}\"}" > /challenge/metadata.json


FROM node:18.19.0 AS challenge

ENV FLAG1="picoCTF{p4ch1nk0_f146_0n3_e947b9d7}"
ENV FLAG2="picoCTF{p4ch1nk0_r3v15173d_flag_two_a6c19d0d}"

WORKDIR /app

# Copy package files
COPY server/package*.json ./

# Install dependencies
RUN npm install

# Create verilog directory and copy cpu.json
RUN mkdir -p /verilog
COPY verilog/cpu.json /verilog/

# Copy the rest of the application
COPY server/ .

# Expose the port the app runs on
EXPOSE 3000
# PUBLISH 3000 AS web

# Start the application
CMD ["node", "index.js"] 
