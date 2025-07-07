#!/bin/bash
# set -x
nix-build
sudo cp -r ./result/* /usr/share/nginx/html
sudo chmod -R a+r /usr/share/nginx/html

# sudo chmod -R a+r ./result/
