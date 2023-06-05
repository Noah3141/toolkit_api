
# Use a base image with the latest version of Rust installed
FROM rust:latest

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG SECRET_KEY
ENV SECRET_KEY=$SECRET_KEY

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Specify the command to run when the container starts
CMD ["./target/release/toolkit_api"]