### ADC + UART TX EXAMPLE

For Adafruit ItsyBitsy nRF52840 Express board (https://learn.adafruit.com/adafruit-itsybitsy-nrf52840-express)

Read a value from an analog pin, output to serial port and blink the LED.

To read the serial port output, use FTDI-USB breakout board, with the FTDI TX pin connected to ItsyBitsy RX pin, and RX to TX.
Picocom is a good simple software for serial communication, and it's set to 9600 bps by default. 

### HOW TO FLASH:

* build the code: ```cargo build --release```
* convert to .hex file: ```arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/blinky blinky.hex```
* create a dfu package: ```adafruit-nrfutil dfu genpkg --dev-type 0x0052 --application blinky.hex blinky.zip```
* put the board into bootloader mode (double click on reset button, will show up as _ITSY840BOOT_ or similar)
* flash the firmware: ```adafruit-nrfutil dfu serial --package blinky.zip -p /dev/ttyACM0 -b 115200```
