#!/bin/bash
REPO_ROOT=$(dirname $(readlink -f $0))/../

${REPO_ROOT}/target/release/mangata-node benchmark \
    --chain dev \
    --execution wasm \
    --wasm-execution compiled \
    --pallet pallet_xyk \
    --extrinsic '*' \
    --steps 20 \
    --repeat 10 \
    --output ./pallets/xyk/src/weights.rs \
    --template ./templates/module-weight-template.hbs
