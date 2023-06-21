
# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .


ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG CHATGPT_API_KEY
ENV CHATGPT_API_KEY=$CHATGPT_API_KEY
# ARG SECRET_KEY
# ENV SECRET_KEY=$SECRET_KEY

# Build the Rust application
RUN cargo build --release

# Specify the command to run when the container starts
CMD ["./target/release/toolkit_api"]