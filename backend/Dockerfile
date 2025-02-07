# Build stage
FROM rust

WORKDIR /usr/scr/app
# Copy the source code
COPY . .

EXPOSE 8080

# Build the application
CMD cargo build --release

CMD cargo run
