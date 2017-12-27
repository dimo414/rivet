#!/bin/bash -eu
#
# Copyright 2017 Google LLC, Matthew Vilim
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

#!/bin/bash
#
# Validates runtime behavior of Rivet, ensuring that standard requests don't trigger panics.
# Note this intentionally does not validate the responses to these requests (even that they 200), the intent here is
# simply to confirm the server stays up.

PORT=8000
HOST='localhost'

PATHS=(
  '/'
  '/xyz' # notice this will 404
  '/raw/foo/bar?baz'
  '/stringly/foo/bar?baz'
  '/pattern/foo/bar?baz'
  '/closure/parts/bar?baz'
  '/closure/paths/bar?baz'
  '/closure/both/bar?baz'
  '/traits/bar?baz'
  '/traits_macro/all/bar?baz'
  '/factory/both/foo?bar'
)

expect() {
  while (( $# > 0 )); do
    if ! which "$1" &> /dev/null; then
      echo "$1 not found, exiting."
      exit # TODO non-zero exit code
    fi
    shift
  done
}

make_requests() {
  # wait for server to start up
  while ! nc -z "$HOST" "$PORT" &> /dev/null; do
    sleep 1;
  done;
  # Create URLs
  urls=()
  for url in ${PATHS[@]}; do
    urls+=("http://${HOST}:${PORT}${url}")
  done
  # shut down the server
  urls+=("http://${HOST}:${PORT}/quit")

  wget --quiet --spider "${urls[@]}"
}

expect nc wget

make_requests &
trap 'kill %1' EXIT

# Build and bring up server, make_requests will send requests once it's up
RUST_BACKTRACE=1 cargo run
