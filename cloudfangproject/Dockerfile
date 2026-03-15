# Stage 1: Build Environment
FROM rust:1.76-slim-bookworm as builder

# Install build dependencies (e.g. for sqlite/reqwest/ssl if needed)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/cloudfang

# Copy the entire workspace
COPY . .

# Build the release binary
RUN cargo build --release -p cloudfang-cli

# Stage 2: Runtime Environment
FROM debian:bookworm-slim

# Install runtime dependencies required for OpenStack calls and SQLite
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

# Set up non-root user for security
RUN useradd -ms /bin/bash cloudfanguser
USER cloudfanguser

WORKDIR /home/cloudfanguser/app

# Copy binary from builder
COPY --from=builder /usr/src/cloudfang/target/release/cloudfang /usr/local/bin/cloudfang
COPY --from=builder /usr/src/cloudfang/cloudfang.toml.example ./cloudfang.toml

# Expose no ports (Since it's a daemon/agent script)

# Initialize the db on launch then start the autonomous daemon
CMD ["sh", "-c", "cloudfang init && cloudfang start"]
