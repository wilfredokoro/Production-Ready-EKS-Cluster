# Use the official Rust image as a builder stage
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# copy the current directory to the WORKDIR
COPY . .

# build and install the app
RUN cargo install --path .

# a little clean up in leu of building a production container
RUN rm -r src
RUN rm -r target

# Expose the necessary port
EXPOSE 8080

# Command to run the application
CMD ["serve", "-s", "build"]
