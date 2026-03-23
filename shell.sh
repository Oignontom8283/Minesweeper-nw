#!/bin/bash

# Allow local connections to the X server
xhost +local:docker

docker exec -it minesweeper_nw bash
