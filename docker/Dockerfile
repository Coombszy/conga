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
RUN ./cross-compile.sh ${TARGETPLATFORM} conga
RUN rm src/*.rs

# Build conga
COPY ./src ./src
ARG TARGETPLATFORM
RUN ./cross-compile.sh ${TARGETPLATFORM} conga
RUN ls -ltra /conga/target/release

# Create image
FROM --platform=$TARGETPLATFORM ubuntu:latest

# Copy binary and sample data/config
WORKDIR /app
COPY --from=build /conga/target/release/conga .
COPY ./data ./data 
COPY ./config ./config
RUN apt-get update && apt-get upgrade -qq && apt-get install -y ca-certificates
RUN update-ca-certificates

# Set entry to run conga
CMD ["./conga"]

########################################################################################################
#   Copyright (C) 2022 Coombszy
#
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <https://www.gnu.org/licenses/>.
