FROM gcc:latest

# Install additional useful packages
RUN apt-get update && apt-get install -y \
    build-essential \
    gdb \
    valgrind \
    && rm -rf /var/lib/apt/lists/*

# Create code directory
RUN mkdir -p /code

# Set working directory
WORKDIR /code

# Default command - we'll compile and run C++ code using a temporary file
CMD ["bash", "-c", "cat > /tmp/program.cpp && g++ -std=c++17 -Wall -Wextra -o /tmp/program /tmp/program.cpp && /tmp/program"] 