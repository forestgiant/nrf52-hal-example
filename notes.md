# Soft Stack

## Obtaining the Soft Device
Get it from [here](https://www.nordicsemi.com/Software-and-Tools/Software/S132/Download)

## Soft Device Manual
### Flash
The combined flash usage of the SoftDevice and the MBR can be found in the SoftDevice properties section of the release notes. This value corresponds to APP_CODE_BASE in Figure 24: Memory resource map on page 68. The combined flash usage of the SoftDevice and the MBR can also be calculated by adding the MBR flash usage, which is 4 kB 11 , to the SD_FLASH_SIZE defined in nrf_sdm.h.

### RAM
- APP_RAM_BASE = 0x20000000 + SoftDevice RAM consumption
  - In our case this should be 0x20000000 + 0x1628 = 0x20001628

## 6.1.1 Release Notes
### SoftDevice Properties
- This SoftDevice is production tested for nRF52832.
- This SoftDevice contains the Master Boot Record (MBR) version 2.4.1 (DRGN-10680).
- This MBR version is compatible with previous MBR versions.
- The combined MBR and SoftDevice memory requirements for this version are the same as for the s132_nrf52_6.1.0:
- Flash: 152 kB (0x26000 bytes).
- RAM: 5.54 kB (0x1628 bytes). This is the minimum required memory. The actual requirements depend on the configuration chosen at sd_ble_enable() time.
- The Firmware ID of this SoftDevice is 0x00B7


## memory.x 

[cortex-m-rt docs](https://rust-embedded.github.io/cortex-m-rt/0.6.1/cortex_m_rt/index.html)

```
MEMORY
{
  FLASH : ORIGIN = 0x26000, LENGTH = 0x5A000
  RAM : ORIGIN = 0x20001628, LENGTH = 0x69D8
}
```

## Flash the softdevice to the nrf52-dk via JLINK
[Jame's Munns doc](https://github.com/jamesmunns/nrf52dk-sys#flashing-the-softdevice)

- `$> cd /path/to/softdevice`
- `$> sudo JLinkExe -device NRF52832_XXAA -if SWD -speed 4000 -autoconnect 1`
- `J-Link> loadfile s132_nrf52_6.1.1_softdevice.hex`
