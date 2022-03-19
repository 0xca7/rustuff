#!/bin/bash

# start openocd and connect to target
openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
