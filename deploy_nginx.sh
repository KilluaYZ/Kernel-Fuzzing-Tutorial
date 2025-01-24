#!/bin/bash
set -x
nix-build
sudo cp -r ./result/* /var/www/html/
sudo chmod -R a+r /var/www/html
