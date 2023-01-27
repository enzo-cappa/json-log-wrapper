#!/bin/bash

cross build -r --target x86_64-unknown-linux-musl
docker build . -t logger
