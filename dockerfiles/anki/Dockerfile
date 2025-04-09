FROM --platform=$BUILDPLATFORM rust AS builder
SHELL ["/bin/bash", "-uo", "pipefail", "-c"]

ARG version
ARG TARGETPLATFORM

# add the rust target for the target architecture
RUN if   [ "$TARGETPLATFORM" == "linux/amd64"  ]; then echo "x86_64-unknown-linux-musl"      >/.target; \
    elif [ "$TARGETPLATFORM" == "linux/arm64"  ]; then echo "aarch64-unknown-linux-musl"     >/.target; \
    elif [ "$TARGETPLATFORM" == "linux/arm/v7" ]; then echo "armv7-unknown-linux-musleabihf" >/.target; \
    else echo "Unknown architecture $TARGETPLATFORM"; exit 1; \
    fi
RUN rustup target add "$(cat /.target)"

COPY cargo-config.toml $CARGO_HOME/config.toml
RUN echo "$CARGO_HOME/config.toml"
RUN dpkg --add-architecture arm64
RUN dpkg --add-architecture armhf
# armv7 => armhf
RUN apt-get -y update && apt-get -y install protobuf-compiler clang mold musl musl-dev musl-dev:arm64 musl-dev:armhf musl-tools
ENV PROTOC=/usr/bin/protoc
RUN echo "build anki version ${version} for $(cat /.target)"
RUN cargo install --locked --target "$(cat /.target)" --git https://github.com/ankitects/anki.git --tag ${version} anki-sync-server
#RUN ldd /usr/local/cargo/bin/anki-sync-server


FROM --platform=$BUILDPLATFORM scratch
ENV SYNC_BASE=/data
VOLUME "$SYNC_BASE"
COPY --from=builder --chmod=0755 /usr/local/cargo/bin/anki-sync-server /
CMD ["./anki-sync-server"]
