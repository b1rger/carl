#!/bin/sh
# SPDX-FileCopyrightText: 2025 Birger Schacht <birger@rantanplan.org>
#
# SPDX-License-Identifier: MIT

set -e
set -x

TESTDIR=$(dirname "$0")
DATADIR=$(dirname "$0")/../data
MKTEMP="mktemp --suffix _carl"

TMPHOME=$($MKTEMP -d)
CONFIG=$TMPHOME/.config/carl
mkdir -p $CONFIG
echo $CONFIG
cp $DATADIR/config.toml $DATADIR/carl.ics $DATADIR/default.theme $CONFIG
sed -i "s#file = \"carl.ics\"#file = \"$CONFIG/carl.ics\"#" $CONFIG/config.toml
cat $CONFIG/*

FAKETIME='2025-09-09 17:47:30'

cargo build --release

for file in $(find $TESTDIR -name '*.test'); do
  ARGUMENTS=$(head -n 1 $file)
  TESTAGAINSTFILE=$($MKTEMP)
  tail -n +2 $file > $TESTAGAINSTFILE
  TESTOUTPUT=$($MKTEMP)
  XDG_CONFIG_HOME=$TMPHOME/.config faketime "$FAKETIME" ./target/release/carl $ARGUMENTS > $TESTOUTPUT
  diff $TESTOUTPUT $TESTAGAINSTFILE
  rm -f $TESTOUTPUT $TESTAGAINSTFILE
done
