#!/bin/sh

sed -i "/<!-- replace start -->/,/<!-- replace end -->$/s|${PREV_VERSION}|${NEW_VERSION}|g" README.md
