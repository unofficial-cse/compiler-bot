FROM gcc:latest

# Install additional useful packages
RUN apt-get update && apt-get install -y \
    build-essential \
    gdb \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /

# Default command - we'll compile and run C++ code using a temporary file
CMD ["bash"]