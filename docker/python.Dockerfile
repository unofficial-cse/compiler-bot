FROM python:3.11-slim

# Install common packages
RUN pip install --no-cache-dir numpy pandas requests matplotlib

# Set working directory
WORKDIR /

# Set environment variables for better Python behavior
ENV PYTHONUNBUFFERED=1
ENV PYTHONIOENCODING=utf-8

# Default command (will be overridden)
CMD ["python3"]