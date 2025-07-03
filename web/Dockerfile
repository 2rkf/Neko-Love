# Use Node.js official image
FROM node:22.12.0

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Build Nuxt 3 app
RUN npm run build

# Expose port
EXPOSE 3000

# Start Nuxt server
CMD ["npm", "run", "start"]