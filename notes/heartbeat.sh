#!/bin/bash

socat STDIN UDP4-SENDTO:192.168.168.55:48689,sourceport=48689 < inithdtx
