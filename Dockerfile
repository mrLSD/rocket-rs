FROM buildpack-deps:jessie-scm
# FROM ubuntu:16.04
COPY target/release/rocket-rs /rocket-rs
ENV ROCKET_ENV=prod
CMD ["./rocket-rs"]

EXPOSE 80