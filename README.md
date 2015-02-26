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

## alternative

shell script by max_plenert:

```
adb shell screencap /sdcard/mytmp/rock.raw
adb pull /sdcard/mytmp/rock.raw
adb shell rm /sdcard/mytmp/rock.raw

// remove the header
tail -c +13 rock.raw > rock.rgba

// extract width height and pixelformat:
hexdump -e '/4 "%d"' -s 0 -n 4 rock.raw
hexdump -e '/4 "%d"' -s 4 -n 4 rock.raw
hexdump -e '/4 "%d"' -s 8 -n 4 rock.raw

convert -size 480x800 -depth 8 rock.rgba rock.png
```
