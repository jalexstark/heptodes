#!/bin/bash -eu
#
# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

#!/bin/sh

# Colour def.
# 0 32 #fffcf4
# 0 33 #0c1438

LIGHT=f8f4d0
DARK=2c3458

cat Packet.fig | sed -e "s/fffcf4/XXXXXX/;s/0c1438/${LIGHT}/;s/XXXXXX/${DARK}/;" > PacketInv.fig

FIGS="Packet PacketInv"
for FILE in Packet PacketInv
do
   fig2dev -L png -m 1.0 ${FILE}.fig > ${FILE}-480x480.png
   fig2dev -L svg -m 1.0 ${FILE}.fig > ${FILE}-6inx6in.svg
   
   fig2dev -L svg -m 0.5 ${FILE}.fig > ${FILE}-3inx3in.svg
   fig2dev -L svg -m 0.3333 ${FILE}.fig > ${FILE}-2inx2in.svg
   # Slight rounding down or it becomes 73x73pt.
   fig2dev -L svg -m 0.1666 ${FILE}.fig > ${FILE}-1inx1in.svg
   
   pngtopnm ${FILE}-480x480.png | pnmscale -reduce 30 | pnmtopng > ${FILE}-16x16.png
   pngtopnm ${FILE}-480x480.png | pnmscale -reduce 20 | pnmtopng > ${FILE}-24x24.png
   pngtopnm ${FILE}-480x480.png | pnmscale -reduce 15 | pnmtopng > ${FILE}-32x32.png
   pngtopnm ${FILE}-480x480.png | pnmscale -reduce 10 | pnmtopng > ${FILE}-48x48.png
   pngtopnm ${FILE}-480x480.png | pnmscale -reduce  5 | pnmtopng > ${FILE}-96x96.png
done
