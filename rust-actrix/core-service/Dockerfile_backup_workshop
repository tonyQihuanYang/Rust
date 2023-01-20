# Rust as the base image
FROM rust:1.66 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin core-service
WORKDIR /core-service

# Copy our manifests
# # Copy './server'
# COPY ./server ./server

# # copy './database'
# COPY ./database ./database

# Copy Cargo.lock && Cargo.toml from './database'
COPY ./database/Cargo.lock ./database/Cargo.lock
COPY ./database/Cargo.toml ./database/Cargo.toml
# Copy Cargo.lock && Cargo.toml from './server'
COPY ./server/Cargo.lock ./server/Cargo.lock
COPY ./server/Cargo.toml ./server/Cargo.toml
# Copy Cargo.lock && Cargo.toml from './'
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy and create Dummy Rust src for cargo build --release
COPY ./docker_src ./server/src
COPY ./docker_src ./database/src

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm -rf ./database
RUN rm -rf ./server

# Copy the source code
COPY ./database ./database
COPY ./server ./server
COPY ./.env ./.env
COPY ./wait_for_it.sh ./wait_for_it.sh

# Build for release.
RUN rm ./target/release/deps/server*
RUN cargo build --release

# The final base image
# FROM debian:bullseye-slim
# Trys to use slim bullseye
FROM rust:1.66-slim-bullseye

# Copy from the previous build
COPY --from=builder /core-service/.env /usr/src/.env
COPY --from=builder /core-service/wait_for_it.sh /usr/src/wait_for_it.sh
COPY --from=builder /core-service/target/release/server /usr/src/server

# Run the installed missed
# CMD sudo yum install postgresql-devel
# Run the binary
CMD /usr/src/server
