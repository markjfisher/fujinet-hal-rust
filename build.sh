#!/bin/bash

# of course there's a build.sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

DO_BUILD=0
DO_CLEAN=0
DO_FLASH=0
EXECUTE_APP=0
IS_RELEASE=0
SHOW_MONITOR=0

RELEASE_STRING=""
TARGET="debug"

function show_help {
  echo "Usage: $(basename $0) [options]"
  echo " -b          # run build"
  echo " -c          # clean project"
  echo " -f          # flash device"
  echo " -m          # run monitor"
  echo " -r          # make release build (default debug)"
  echo " -x          # execute application"
  exit 1
}


if [ $# -eq 0 ] ; then
  show_help
fi

while getopts "bcfhmrx" flag
do
  case "$flag" in
    b) DO_BUILD=1 ;;
    c) DO_CLEAN=1 ;;
    m) SHOW_MONITOR=1 ;;
    f) DO_FLASH=1 ;;
    r) IS_RELEASE=1 ;;
    x) EXECUTE_APP=1 ;;
    h) show_help ;;
    *) show_help ;;
  esac
done
shift $((OPTIND - 1))

if [ $IS_RELEASE -eq 1 ] ; then
  RELEASE_STRING="--release"
  TARGET="release"
fi

if [ $DO_CLEAN -eq 1 ] ; then
  cargo clean
fi

if [ $DO_BUILD -eq 1 ] ; then
  cargo build ${RELEASE_STRING}
  if [ $? -ne 0 ] ; then
    echo "Failed to build, exiting"
    exit 1
  fi
  cp ${SCRIPT_DIR}/target/xtensa-esp32s3-espidf/${TARGET}/bootloader.bin ${SCRIPT_DIR}/target/
fi

if [ $DO_FLASH -eq 1 ] ; then
  cargo espflash flash
fi

if [ $SHOW_MONITOR -eq 1 ] ; then
  cargo espflash monitor
fi

if [ $EXECUTE_APP -eq 1 ] ; then
  cargo run ${RELEASE_STRING}
fi
