# Use Node.js official image
FROM node:22.12.0

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

# Build Vuepress site
RUN npm run build

# Expose port
EXPOSE 3001

# Serve the built site
CMD ["npm", "run", "docs:serve"]