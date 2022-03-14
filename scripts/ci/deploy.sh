#!/bin/bash
set -exu

# for remote_host in ["idp_test", "a"]


# deploy to 82.xxx.xxx.xxx machine
cargo b --release --target x86_64-unknown-linux-musl
scp ddl.sql idp_test:~/deploy
ssh idp_test "systemctl --user stop idp_shop_demo"
scp target/x86_64-unknown-linux-musl/release/idp_shop_demo idp_test:~/deploy
ssh idp_test "systemctl --user start idp_shop_demo"
