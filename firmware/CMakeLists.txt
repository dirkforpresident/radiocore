# RadioCore Firmware — ESP32-S3
# Requires ESP-IDF v5.x

cmake_minimum_required(VERSION 3.16)

set(EXTRA_COMPONENT_DIRS $ENV{IDF_PATH}/examples/peripherals/usb/device/tusb_composite_msc_serialdevice/components)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(radiocore-fw)
