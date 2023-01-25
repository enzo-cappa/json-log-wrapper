FROM alpine:latest

RUN apk add rsyslog bash
ADD rsyslog.conf /etc/rsyslog.conf
ADD tojson.bin /usr/local/bin

# ENTRYPOINT ["rsyslogd", "-n"]
