FROM node:slim

ENV NODE_ENV production

WORKDIR /express-docker

COPY . .

RUN npm install

RUN npm install pm2 -g

CMD pm2 start process.yml && tail -f /dev/null

EXPOSE 8000