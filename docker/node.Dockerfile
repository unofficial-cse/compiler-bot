FROM node:lts-slim

# Install common packages (if needed in the future)
# RUN npm install -g typescript ts-node

# Create code directory
RUN mkdir -p /code

# Set working directory
WORKDIR /code

# Set environment variables
ENV NODE_ENV=sandbox

# Default command - reads and executes JavaScript from stdin
CMD ["node"]