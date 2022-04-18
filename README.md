# dcc_flutter_raspi_pico
dcc demo of android app (flutter) and raspberry pi pico using rust

youtube demo -->  https://www.youtube.com/watch?v=FUroU-ptDjU

see rust/main.rs for hookup to the hc-19 ble module and drv8801 dcc booster

*****************************************
* flutter android app main file readme: *
*****************************************

// demo of flutter dcc using bluetooth

// run on pixel 4, android version 12

// flutter version - 2.13.0-0.1.pre - channel dev

// dart - 2.17.0, devtools 2.12.1

// change android/app/build.gradle to minSdkVersion 21

// 

// flutter install on fresh linux liteos 

//    -- https://www.linuxliteos.com/

//

/*
flutter ble template and flutter linux install instruction

flutter install on linux howto: https://ubuntu.com/blog/getting-started-with-flutter-on-ubuntu

sudo snap install flutter --classic

sudo snap alias flutter.dart dart

sudo snap install android-studio --classic

launch android studio:

click more actions (lower part of the screen)

click on SDK Manager

under SDK Platfroms tab add any Android you want

under SDK Tools tab

    click on:
    
        Android SDK Command-line Tools
        
        Android Emulator
        
        Android SDK Platforms-Tools
        
        click Apply

flutter config --android-studio-dir /snap/android-studio/current/android-studio

flutter doctor --android-licenses

flutter channel dev

flutter upgrade

flutter config --enable-linux-desktop

flutter doctor

mkdir flutter_projects

cd flutter_projects

flutter create myapp

cd myapp

flutter run -d linux (this will run on your desktop)
*/

********************************************
* raspberry pi pico rust main file readme: *
********************************************

// demo of rust pico dcc using bluetooth

// dcc booster is hooked up on gpio 15

// 

// hc-19 ble vcc - pico 5v, ground - pico any ground pin

// hc-19 tx - pico gpio13

// hc-19 rx - pico gpio12

// I changed the baud rate for the hc-19 to 115200

//

// rust install on a fresh linux liteos

//    -- https://www.linuxliteos.com/

//

/*

sudo apt install git gdb-multiarch
    
sudo apt install automake autoconf build-essential texinfo libtool libftdi-dev libusb-1.0-0-dev libudev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc -- version (to check if rust installs successfully)

rustup update

rustup target install thumbv6m-none-eabi (this is for pico)

cargo install flip-link

cargo install elf2uf2-rs --locked

*/



