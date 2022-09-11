FROM --platform=$BUILDPLATFORM rust:slim as build

# Creates empty project, to cache dependancies
# Highlights upstream dependancy errors
RUN USER=root cargo new --bin conga
WORKDIR /conga
COPY ./Cargo.toml ./Cargo.toml
COPY ./scripts/cross-compile.sh ./cross-compile.sh
COPY ./.cargo/config ./.cargo/config
RUN chmod +x ./cross-compile.sh
ARG TARGETPLATFORM
RUN ./cross-compile.sh ${TARGETPLATFORM}
RUN rm src/*.rs

# Build conga
COPY ./src ./src
ARG TARGETPLATFORM
RUN ./cross-compile.sh ${TARGETPLATFORM}
RUN ls -ltra /conga/target/release

# Create image
FROM ubuntu:latest

# Copy binary and sample data/config
WORKDIR /app
COPY --from=build /conga/target/release/conga .
COPY ./data ./data 
COPY ./config ./config
RUN apt-get update && apt-get upgrade -qq && apt-get install -y ca-certificates
RUN update-ca-certificates

# Set entry to run conga
CMD ["./conga"]
