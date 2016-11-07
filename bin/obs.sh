#!/usr/bin/env bash
# Usage bin/obs.sh <log-level>
log_level=debug

if [ -n "$1" ]; then
	log_level=$1
fi

export RUST_BACKTRACE=1
export RUST_LOG=obs_twitch_dashboard=$log_level
exec obs
