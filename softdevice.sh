#!/bin/sh

echo "After connecting run \"loadfile s132_nrf52_6.1.1_softdevice.hex\""
sudo JLinkExe -device NRF52832_XXAA -if SWD -speed 4000 -autoconnect 1
