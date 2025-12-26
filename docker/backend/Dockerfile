ARG ENVIRONMENT_MODE=production
FROM rust:1.92-alpine

ARG TIMEZONE=UTC
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apk add --no-cache tzdata openssl openssl-dev openssl-libs-static pkgconfig

RUN ln -sf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime
RUN echo ${TIMEZONE} > /etc/timezone

RUN addgroup -g ${GROUP_ID} matic
RUN adduser -D -u ${USER_ID} -G matic matic
RUN mkdir -p /usr/src/matic
RUN chown -R matic:matic /usr/src/matic

WORKDIR /usr/src/matic
USER matic

RUN cargo install sqlx-cli
RUN cargo install cargo-watch

# ENTRYPOINT ["tail"]
# CMD ["-f","/dev/null"]
CMD ["cargo", "watch", "-x", "run"]
