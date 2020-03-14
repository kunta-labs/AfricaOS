#FROM rust:latest
#FROM rust:1.36.0 AS build
FROM clux/muslrust AS build

# TODO: map nodetype
ARG nodeType

RUN mkdir /environment/

COPY ./core /environment/

WORKDIR /environment/

RUN rustup target add x86_64-unknown-linux-musl

# just to show it builds, may not need
RUN cargo build --release

# Copy the source and build the application.
COPY ./core/src ./src

####

####
#RUN apt-get install pkg-config libx11-dev libxmu-dev

RUN cargo install --target x86_64-unknown-linux-musl --path .

################second stage
FROM scratch

# test ports for a, b, and c
EXPOSE 8081
EXPOSE 8082
EXPOSE 8083

#test
#COPY --from=build /usr/local/cargo/bin/core .

#works, copy binary
COPY --from=build /environment/target/x86_64-unknown-linux-musl/release/core .

# copy storage directory
COPY --from=build /environment/storage/ .
COPY --from=build /environment/storage/chain ./storage/chain
COPY --from=build /environment/storage/transaction ./storage/transaction
COPY --from=build /environment/storage/proposal ./storage/proposal
COPY --from=build /environment/storage/state ./storage/state

#ADD --from=build /environment/storage/ ./storage

###############
#CMD ["./core"]
