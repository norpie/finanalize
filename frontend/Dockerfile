# base image
FROM node:18

# set working directory
WORKDIR /app

# copy package files
COPY package.json package-lock.json ./

# install dependencies
RUN npm install

# copy the rest of the application files
COPY . .

# expose the port sveltekit uses
EXPOSE 5173

# run the dev server
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
