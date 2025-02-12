# Use Node.js base image
FROM node:18

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json to install dependencies
COPY package.json ./
RUN npm install

# Copy the application code
COPY . .

# Build the React app
RUN npm run build

# Install a static file server for production
RUN npm install -g serve

# Expose the application port
EXPOSE 3000

# Start the static server
CMD ["serve", "-s", "build"]
