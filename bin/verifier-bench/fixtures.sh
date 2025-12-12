#!/bin/bash

BLOCK_NUMBER=${BLOCK_NUMBER:-23992138}
APP_LOG_BLOWUP=${APP_LOG_BLOWUP:-1}
LEAF_LOG_BLOWUP=${LEAF_LOG_BLOWUP:-1}
S3_FOLDER="s3://axiom-public-data-sandbox-us-east-1/benchmark/github/fixtures/reth-app${APP_LOG_BLOWUP}-leaf${LEAF_LOG_BLOWUP}-${BLOCK_NUMBER}"

mkdir -p fixtures
s5cmd  --no-sign-request cp "${S3_FOLDER}/app_proof.bitcode" fixtures
s5cmd  --no-sign-request cp "${S3_FOLDER}/leaf_proofs.bitcode" fixtures
s5cmd  --no-sign-request cp "${S3_FOLDER}/app_pk.bitcode" fixtures
s5cmd  --no-sign-request cp "${S3_FOLDER}/agg_pk.bitcode" fixtures
