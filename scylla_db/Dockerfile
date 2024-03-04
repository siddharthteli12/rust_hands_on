# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this layer will be cached unless dependencies change)
RUN cargo build --release

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/scylla_db .

# Expose the port your Actix web server listens on
EXPOSE 8080

# Command to run the application
CMD ["./scylla_db"]
