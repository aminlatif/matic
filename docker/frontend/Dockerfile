ARG ENVIRONMENT_MODE=production
FROM node:lts AS base

ARG TIMEZONE=UTC
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update -y && apt-get upgrade -y && apt-get clean

RUN ln -sf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime
RUN echo ${TIMEZONE} > /etc/timezone

RUN mkdir -p /usr/src/matic
RUN chown -R node:node /usr/src/matic

RUN npm install -g pnpm@latest-10

WORKDIR /usr/src/matic
USER node

# ENTRYPOINT ["tail"]
# CMD ["-f","/dev/null"]
CMD ["pnpm", "dev", "-o"]
