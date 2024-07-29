FROM rust:1.75 as builder
WORKDIR /usr/src/myapp

COPY . .

ADD ./infrastructure/certs/crates.io.crt /usr/local/share/ca-certificates/crates.io.crt
# ADD ./infrastructure/certs/mob_crates.io.crt /usr/local/share/ca-certificates/mob_crates.io.crt
ADD ./infrastructure/certs/static.crates.io.crt /usr/local/share/ca-certificates/static.crates.io.crt
# ADD ./infrastructure/certs/mob_static.crates.io.crt /usr/local/share/ca-certificates/mob_static.crates.io.crt

RUN apt install ca-certificates && \
    update-ca-certificates

RUN chmod 644 /usr/local/share/ca-certificates/crates.io.crt \
    # && chmod 644 /usr/local/share/ca-certificates/mob_crates.io.crt \
    # && chmod 644 /usr/local/share/ca-certificates/mob_static.crates.io.crt \
    && chmod 644 /usr/local/share/ca-certificates/static.crates.io.crt && update-ca-certificates

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/webserver /usr/local/bin/webserver
COPY --from=builder /usr/local/cargo/bin/rtdomain /usr/local/bin/rtdomain
COPY --from=builder /usr/local/cargo/bin/pmgr /usr/local/bin/pmgr
COPY --from=builder /usr/local/cargo/bin/pmgr /usr/local/bin/pmgr
COPY --from=builder /usr/src/myapp/infrastructure/scripts/*.* /usr/local

WORKDIR /usr/local

EXPOSE 7878
EXPOSE 3000
ENV FTP_CONN=host.docker.internal
# RUN chmod +x ./installshiftdomain.sh
# ENTRYPOINT ["./installshiftdomain.sh"]
CMD ["./bin/pmgr", "/usr/local/processmgr.toml"]
# CMD ["./infrastructure/scripts/installshiftdomain.sh"]
#CMD ["webserver"]