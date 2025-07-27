FROM node:lts-slim

# Install common packages (if needed in the future)
# RUN npm install -g typescript ts-node

# Set working directory
WORKDIR /

# Set environment variables
ENV NODE_ENV=sandbox

# Default command - reads and executes JavaScript from stdin
CMD ["node"]