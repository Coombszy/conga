#!/bin/bash


echo "TARGETPLATFORM: $1"

apt-get update && apt-get upgrade -qq
apt-get install --install-recommends -y perl openssl libssl-dev librust-openssl-dev \
	build-essential ca-certificates

if [ "$1" = "linux/arm/v7" ] 
then
    rm -f target/armv7-unknown-linux-gnueabihf/release/deps/conga-*
    apt-get install --install-recommends -y gcc-arm-linux-gnueabihf pkg-config
    rustup target add armv7-unknown-linux-gnueabihf
    cargo build --release --target armv7-unknown-linux-gnueabihf; code=$?
    mkdir -p /target/release
    cp target/armv7-unknown-linux-gnueabihf/release/conga target/release/
    exit $code
fi

if [ "$1" = "linux/amd64" ]
then
    # Assuming you are running this on a amd64 machine
    rm -f target/release/deps/conga-*
    cargo build --release
    exit $?
fi

echo "Not supported cross-compile!, Add support in cross-compile.sh and update .cargo/config"
exit 1

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