FROM alpine:latest

ARG SOURCE_DIR=armv7-unknown-linux-muslabi/release
COPY target/${SOURCE_DIR}/weather-to-prometheus /usr/local/bin/weather-to-prometheus

CMD ["/usr/local/bin/weather-to-prometheus"]