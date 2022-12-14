# Awesome gaming mouse
The awesome gaming mouse project, the one mouse to rule them all. The schematics
are available [here](schematics_and_pcb). The firmware for the mouse is written in rust using [rtic](https://rtic.rs/) and is found [here](code/e7020e_rtic) the rtic app is in the example [rtic_simple_mouse](code/e7020e_rtic/examples/rtic_simple_mouse.rs). The program used to configure the mouse is found [here](code/hosted/rust_version).

## How to flash the mouse
To flash the mouse read [this](e7020e-project/code/e7020_rtic/README.md) to get the build dependecies. When the dependencis are in place it can be built, flashed and run with `cargo run --example rtic_simple_mouse --release`. Note! it can't be built in debug mode since this will create to big of a binary and will not fit on the flash of a stm32f401re.

## Members

- Erik Serrander
- eriser-8@student.ltu.se
  
- Ivar Jönsson
- ivajns-9@student.ltu.se

![main schematic](images/main_schematic.png)

## High level specification

### Basic
- [x] Left and right click
- [x] relative position

### Higher
- [x] scroll wheel
- [x] macro system
- [x] Side buttons

### Stretch
- [x] Computer communication
- [ ] LEDs, i.e. debug/ui
- [ ] Rumble motor
- [ ] Perfect spray


### Implementation
- Unit testing
- Cuntinuous integration



# Specifications
## Mouse buttons
The buttons used are [D2F-01F](https://www.elfa.se/sv/mikrobrytare-d2f-100ma-1co-74n-kolv-omron-electronic-components-d2f-01f/p/11082662?track=true&no-cache=true&marketingPopup=false) since these are well regarded by users.
We have decided to have 4 main buttons on the mouse, 2 on the front and 2 on the side.
## Side buttons 
![side_buttons](images/side_buttons.png)
![side_pcb](images/side_button_board.png)
![side_pcb](images/side_button_board_back.png)

To reduce the bouncing of the buttons a 10u capacitor is places to ground from the signal trace. This will allow the signal to be pulled low for high frequencies. 

To drive the buttons a 5v voltage is used with a series resistor of 4.7 k to limit the current to close to 1mA. The mosfet used to limit the output acts as a decoupler thus not draining anny current ( in theory ).

## Front buttons
![front_buttons](images/front_buttons.png)
## Mouse wheel


![mouse_wheel](images/mouse_wheel.png)

The [mouse wheel](https://se.rs-online.com/web/p/mechanical-rotary-encoders/7295545) is also debounced using the same capacitor values as the mouse button. This should not be as needed but better safe than sorry.

The mouse wheel also uses the same nfet to step down the voltage.


## Rumble motor
![Rumble motor image](images/rumble_motor.png)


## Main board
![main board](images/mainboard_front.jpg)
![main board back](images/mainboard_back.jpg)

## Case 
We will base our case on this moddel of the [g305](https://www.thingiverse.com/thing:3969266)
with this [base](https://www.thingiverse.com/thing:3564894).


The stl files can be found in our [model folder](model/).


## Grading

- Erik Serrander
  - Expected contributions towards grade 4
    - side buttons
    - diodes
  - Expected contributions towards grade 5
    - Computer communication
    - Rumble motor
  
- Ivar Jönsson
  - Expected contributions towards grade 4
    - PCB layout
      - Itterative design using a 3d printer to validate pcb fitting and footprint layout
      - Ease of assembly
    - Case customization 
      - Mountingholes and posibility to use a usb cable
    - Mouse wheel
      - Selecting a rotary encoder
    - diodes
      - status LEDs to validate voltage sections
  - Expected contributions towards grade 5
    - Computer communication
    - DPI configuration
    - Perfect spray
      - This might not be applicable. 


## contributions

- Erik Serrander
  - Hardware design (werry minor)
  - Macro system
  - Macro system config mouse side
  - Mouse state software (major)

- Ivar Jönsson
  - Hardware design (allmost all)
  - Macro system config pc side
  - Mouse state software (minor)
  - Enclosure manufacturing

- Togheter / both
  - Init code
  - Interuppt handlers
  - Harware assembly
