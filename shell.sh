#!/bin/bash

# Allow local connections to the X server
xhost +local:docker

docker exec -it {{project-name}} bash
