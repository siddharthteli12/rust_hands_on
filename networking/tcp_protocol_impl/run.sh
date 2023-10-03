#!/bin/sh
cargo b --release
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp_protocol_impl
$CARGO_TARGET_DIR/release/tcp_protocol_impl &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid
