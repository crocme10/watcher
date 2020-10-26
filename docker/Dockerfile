FROM rust:1.46-buster as builder

RUN USER=root cargo new --lib watcher
WORKDIR ./watcher
COPY ./Cargo.toml ./Cargo.toml

# clang llvm & Co. to build argonautica
RUN apt-get update \
    && apt-get install -y clang llvm-dev libclang-dev \
    && rm -fr /var/lib/apt/list/*

RUN cargo build --release --lib
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/watcher*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/opt/watcher

RUN apt-get update \
    && apt-get install -y netcat \
    && apt-get install -y libssl1.1 \
    && rm -fr /var/lib/apt/list/*

ENV TZ=Etc/UTC \
    RUN_MODE=development \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /watcher/target/release/service ${APP}/bin/service
COPY --from=builder /watcher/config ${APP}/etc
COPY --from=builder /watcher/docker/entrypoint.sh ${APP}/bin/entrypoint.sh

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./bin/entrypoint.sh"]
