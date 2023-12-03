#!/bin/bash

session="advent-of-code-d1"

tmux new-session -d -s $session

window=1
tmux rename-window -t $session:$window "neovim" c-m
tmux send-keys -t $session:$window "v ." c-m

window=2
tmux new-window -t $session:$window -n "test"

tmux attach-session -t $session
