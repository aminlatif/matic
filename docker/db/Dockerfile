ARG ENVIRONMENT_MODE=production
FROM postgres:18-alpine

ARG TIMEZONE=UTC

RUN ln -sf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime
RUN echo ${TIMEZONE} > /etc/timezone
