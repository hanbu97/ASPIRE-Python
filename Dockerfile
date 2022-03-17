FROM alpine

COPY target/x86_64-unknown-linux-musl/release/idp_shop /bin
COPY .env /


ENTRYPOINT [ "/bin/idp_shop" ]



