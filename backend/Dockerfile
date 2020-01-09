# build stage
FROM rust:latest as build

# create new empty binary project
RUN USER=root cargo new --bin tsicket
WORKDIR /tsicket

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy project source and necessary files
COPY ./src ./src

# rebuild app with project source
RUN rm ./target/release/deps/tsicket*
RUN cargo build --release

EXPOSE 8080

RUN cd ./target/release
CMD sudo ./tsicket