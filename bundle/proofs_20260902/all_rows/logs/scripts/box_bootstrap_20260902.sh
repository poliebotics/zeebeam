#!/bin/bash
# Run ON the proving box (Lambda Ubuntu image, user ubuntu) as the first step. Idempotent.
# Installs rustup, the SP1 6.4.0 toolchain, mirrors the absolute paths the crates expect, and
# checks the GPU. The payload itself is pushed from halo by transfer_to_box_20260902.sh.
set -eu
echo "=== box facts ==="; nproc; free -g | head -2; df -h / | tail -1; nvidia-smi --query-gpu=name,memory.total,driver_version --format=csv 2>/dev/null || echo "no GPU visible"
sudo apt-get update -qq && sudo apt-get install -y -qq build-essential pkg-config libssl-dev git curl rsync jq protobuf-compiler > /dev/null  # protoc: sp1-prover-types build script
if ! command -v cargo >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal >/dev/null
fi
source "$HOME/.cargo/env"
if [ ! -x "$HOME/.sp1/bin/cargo-prove" ]; then
  curl -L https://sp1up.succinct.xyz | bash >/dev/null 2>&1
  "$HOME/.sp1/bin/sp1up" --version v6.4.0
fi
export PATH="$HOME/.sp1/bin:$PATH"
cargo prove --version || true
# Go is needed by sp1-sdk's native-gnark feature (the Groth16 wrap without docker)
if ! command -v go >/dev/null; then
  curl -sL https://go.dev/dl/go1.23.4.linux-amd64.tar.gz -o /tmp/go.tgz && sudo tar -C /usr/local -xzf /tmp/go.tgz
fi
export PATH="/usr/local/go/bin:$PATH"; go version
rustup toolchain list | grep -i succinct || echo "succinct toolchain absent"
# the crates carry absolute halo paths: mirror them here, owned by ubuntu
sudo mkdir -p /home/c/Documents/BOSUN/scratch /data/zeebeam_evidence_20260822/stage/live_300s_training_001/Recordings
sudo chown -R ubuntu:ubuntu /home/c /data/zeebeam_evidence_20260822
docker --version || echo "docker absent (needed for the Groth16 gnark wrap)"
echo BOOTSTRAP_DONE
