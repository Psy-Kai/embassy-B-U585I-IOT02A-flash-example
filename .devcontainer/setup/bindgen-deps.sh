#!/bin/bash

readonly DISTRIB_ID=$(awk -F= '/^ID=/{print $2}' /etc/os-release)

apt update

case ${DISTRIB_ID} in
"ubuntu")
  apt install -y \
    libclang-dev
  ;;
"debian")
  apt install -y \
    libclang-19-dev
  ;;
*)
  echo "Unsupported distribution: $DISTRIB_ID"
  exit 1
  ;;
esac
