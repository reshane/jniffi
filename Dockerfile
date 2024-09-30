FROM ubuntu:20.04 AS rs-build

RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev -y

RUN apt-get install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "jniffi"

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --target x86_64-unknown-linux-musl --release 

FROM ubuntu:20.04
RUN apt-get update -y
RUN apt-get install -y openjdk-17-jdk

COPY ./src ./src
RUN mkdir -p ./target/java
RUN javac -h ./target/java ./src/java/HelloWorld.java -d .

COPY --from=rs-build /etc/passwd /etc/passwd
COPY --from=rs-build /etc/group /etc/group

USER jniffi:jniffi

COPY --from=rs-build --chown=jniffi:jniffi ./target/x86_64-unknown-linux-musl/release/ /usr/local/lib/

#ENTRYPOINT ["/bin/sh", "-c", "-l"]
ENTRYPOINT ["java", "-Djava.library.path=/usr/local/lib", "HelloWorld"]

