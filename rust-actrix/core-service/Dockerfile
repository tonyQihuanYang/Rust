# FROM rustlang/rust:nightly


# # Install cargo dependencies
# # RUN cargo install diesel_cli --no-default-features --features postgres
# # RUN cargo install cargo-watch

# # Copy files
# WORKDIR /usr/src/app
# COPY ./ ./

# # Install Dependencies
# RUN cargo build --workspace

# EXPOSE 8080
# VOLUME ["/usr/local/cargo"]
# # CMD ["cargo", "watch", "-c", "-x", "run"]
# # CMD ["cargo", "run"]


# Rust as the base image
FROM rust:1.66 as builder


# == Development ==
# Install cargo dependencies
# RUN cargo install cargo-watch

# Create a new empty shell project
RUN USER=root cargo new --bin core-service
WORKDIR /core-service

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
# == Development ==
# RUN cargo build

# == Production ==
RUN cargo build --release

# Copy the source code
Copy ./ ./

# == Development ==
# Give permission on wait-for-it
# RUN chmod +x ./wait_for_it.sh
# Watch change
# CMD ["cargo", "watch", "-x", "run"]

# == Production ==
# Remove all the related files from this project
RUN rm ./target/release/deps/rust_core_service* 
# Build for release.
RUN cargo build --release

# The final base image
# FROM debian:bullseye-slim
# Trys to use slim bullseye
FROM rust:1.66-slim-bullseye

# Copy from the previous build
COPY --from=builder /core-service/.env /usr/src/.env
COPY --from=builder /core-service/wait_for_it.sh /usr/src/wait_for_it.sh
COPY --from=builder /core-service/target/release/rust-core-service /usr/src/rust-core-service

# Run the installed missed
# CMD sudo yum install postgresql-devel
# Run the binary
CMD /usr/src/rust_core_service
