#!/bin/bash

socat UDP4-RECVFROM:2068,ip-add-membership=226.2.2.2:192.168.168.123 STDOUT
