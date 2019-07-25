# From latest rust stable version
FROM rust:1.36

# Declare args
ARG REVISION
ARG RELEASE_TAG
ARG BUILD_DATE

# image-spec annotations using labels
# https://github.com/opencontainers/image-spec/blob/master/annotations.md
LABEL org.opencontainers.image.created=${BUILD_DATE}
LABEL org.opencontainers.image.source="https://github.com/papey/fediwatcher"
LABEL org.opencontainers.image.revision=${GIT_COMMIT_SHA}
LABEL org.opencontainers.image.version=${RELEASE_TAG}
LABEL org.opencontainers.image.authors="Wilfried OLLIVIER"
LABEL org.opencontainers.image.title="fediwatcher"
LABEL org.opencontainers.image.description="Fediwatcher runtime"
LABEL org.opencontainers.image.licences="Unlicense"

# New empty project
RUN USER=root cargo new --bin fediwatcher
WORKDIR /fediwatcher

# Fetch deps list
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Step to build a default hello world project.
# Since Cargo.lock and Cargo.toml are present,
# all deps will be downloaded and cached inside this upper layer
RUN cargo build --release
RUN rm src/*.rs

# Now, copy source code
COPY ./src ./src

# Build the real project
RUN rm ./target/release/deps/fediwatcher*
RUN cargo build --release

# Finaly, setup command
CMD ["./target/release/fediwatcher"]