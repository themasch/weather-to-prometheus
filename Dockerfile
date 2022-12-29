FROM alpine:latest

COPY target/armv7-unknown-linux-gnueabihf/release/weather-to-prometheus /usr/local/bin/weather-to-prometheus

CMD ["/usr/local/bin/weather-to-prometheus"]