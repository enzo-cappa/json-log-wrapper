FROM alpine:latest

RUN apk add rsyslog bash
ADD rsyslog.conf /etc/rsyslog.conf
ADD target/x86_64-unknown-linux-musl/release/tojson /usr/local/bin

# ENTRYPOINT ["rsyslogd", "-n"]
