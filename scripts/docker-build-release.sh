#!/bin/bash

cd $(git rev-parse --show-toplevel)

APP_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r ".packages[].version")

# Create builder
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx rm builder || true
docker buildx create --name builder --driver docker-container --use
docker buildx use builder

# Run build
docker buildx build --push \
-f docker/Dockerfile --platform linux/amd64,linux/arm/v7,linux/arm64 --tag coombszy/conga:dev --tag coombszy/conga:$APP_VERSION --tag coombszy/conga:latest .

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
