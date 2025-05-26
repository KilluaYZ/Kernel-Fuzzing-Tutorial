#!/bin/bash
set -x
nix-build
sudo cp -r ./result/* /usr/share/nginx/html