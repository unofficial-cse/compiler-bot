FROM node:lts-slim

# Install TypeScript libraries
RUN npm install -g typescript tsx

# Set working directory
WORKDIR /

# Default command - reads and executes JavaScript from stdin
CMD ["tsx"]