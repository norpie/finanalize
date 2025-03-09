#!/usr/bin/env bash

# This script splits the current tmux window into 4 splits
#
# 1 | 2
# -----
# 3 | 4
#
# Each split will be running and instance of `geckodriver` with ports [4444, 4445, 4446, 4447]

tmux split-window -h
tmux select-pane -L
tmux split-window -v
tmux select-pane -R
tmux split-window -v

tmux send-keys -t 2 "trap 'tmux kill-pane' SIGINT" C-m
tmux send-keys -t 3 "trap 'tmux kill-pane' SIGINT" C-m
tmux send-keys -t 4 "trap 'tmux kill-pane' SIGINT" C-m

tmux send-keys -t 1 "geckodriver --port=4444" C-m
tmux send-keys -t 2 "geckodriver --port=4445" C-m
tmux send-keys -t 3 "geckodriver --port=4446" C-m
tmux send-keys -t 4 "geckodriver --port=4447" C-m
