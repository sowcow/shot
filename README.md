# shot

- A little bit faster way to make a screenshot on android device through usb.
- I getting ~2sec, compared to ~6sec if doing default way.
- It works on my older Nexus7.
- Not sure about other devices.
- And you may want to remove 90deg rotation at `ppm_to_bmp` step.

## so

it runs same `adb shell screencap` but w/o png conversion and converts given data to bmp at home

## depends

- imagemagick
- adb, and usb connected device
