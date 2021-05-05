#!/usr/bin/env nix-shell
#!nix-shell -i bash -p bash
cargo build --release
status=$?
if [[ $status -ne 0 ]]; then
  echo "failed to build"
  exit $status
fi
sudo setcap cap_net_admin=eip target/release/rusty_pipes
target/release/rusty_pipes &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
