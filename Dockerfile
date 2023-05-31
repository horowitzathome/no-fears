# Use the official rust image for Apple Silicon (M1)
FROM rustlang/rust:nightly as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the Rust program
RUN cargo build --release    

# Use a minimal Alpine Linux image
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy missing dependencies
COPY --from=builder /lib/aarch64-linux-gnu/libgcc_s.so.1 /lib
COPY --from=builder /lib/aarch64-linux-gnu/libpthread.so.0 /lib
COPY --from=builder /lib/aarch64-linux-gnu/libdl.so.2 /lib
COPY --from=builder /lib/aarch64-linux-gnu/libc.so.6 /lib
COPY --from=builder /lib/aarch64-linux-gnu/ld-linux-aarch64.so.1 /lib

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/no-fears .

# Set the entry point command
CMD ["/app/no-fears"]
