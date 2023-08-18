### KVM Virtual Machine Serial Port Configuration
Add Serial 
```xml
<serial type="unix">
  <source mode="bind" path="/tmp/gt_com"/>
  <target type="isa-serial" port="1">
    <model name="isa-serial"/>
  </target>
  <alias name="serial1"/>
</serial>     
```

```bash
sudo virsh dumpxml RX-GranuTools | grep -i serial    
```



OLD CODE BELOW
```
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Dummy Flow Sensor
pip install pySerial
find VM pseudo device with
sudo virsh dumpxml Granutools-RX10.3
   <serial type='pty'>
      <source path='/dev/pts/3'/>
      <target type='isa-serial' port='0'>
        <model name='isa-serial'/>
      </target>
      <alias name='serial0'/>
    </serial>
Adjust Serial(/dev/pts/#')
"""

import argparse
from random import randint
from serial import Serial

# ......................................................................................................................
class dummyFlowSensor:
    deviceName = '/dev/pts/3'
    ser: Serial
    inBuf: bytearray = []
    outBuf: bytearray = []

    def __init__(self):
        print('Dummy Flow Sensor')
        pass

    # ......................................................................................................................
    def listen(self):
        self.ser = Serial(self.deviceName)
        print('Listen to Serial:', self.ser.name)
        while 1:
            self.ser.timeout = None
            c = self.ser.read(4)
            if (len(c) == 0):
              continue
            if (c == b'gpwo'):
                print('<< AskWho', c)
                self.prepareAnswer(c)
                self.sendWho()
            elif (c == b'gpfp'):
                print('<< AskValues', c)
                self.prepareAnswer(c)
                self.sendValues()
            else:
                print('<<', c)
                self.ser.timeout = 0.100
                c = self.ser.readline()
                print('**', c)

    @staticmethod
    def calcCRC(aa: bytearray):
        crc: int = 0
        for i in range(1, 23):
            crc = crc + aa[i]
        s: str = str(crc).zfill(4)
        aa[24] = ord(s[0])
        aa[25] = ord(s[1])
        aa[26] = ord(s[2])
        aa[27] = ord(s[3])

    # ......................................................................................................................
    def prepareAnswer(self, cmd:bytes):
        c = self.ser.read(15)
        print('<<', c)
        self.inBuf = bytearray(c)
        self.outBuf = bytearray(29)
        self.outBuf[ 0] = ord('_')
        self.outBuf[ 1] = cmd[0]
        self.outBuf[ 2] = cmd[1]
        self.outBuf[ 3] = cmd[2]
        self.outBuf[ 4] = cmd[3]
        self.outBuf[ 5] = ord('-')
        for i in range(6, 14):
          self.outBuf[i] = ord('0')
        self.outBuf[14] = ord('-')
        for i in range(15, 23):
          self.outBuf[i] = ord('0')
        self.outBuf[23] = ord('-')
        self.outBuf[28] = 13
        pass

    # ......................................................................................................................
    def sendWho(self):
        self.outBuf[13] = ord('6')
        self.outBuf[22] = ord('7')
        self.calcCRC(self.outBuf)
        print('>>', self.outBuf)
        self.ser.write(self.outBuf)

    # ......................................................................................................................
    def sendValues(self):
        f = 6100000 + randint(1, 551024)
        p = 2100000 + randint(1, 551024)
        sf: str = str(f).zfill(8)
        sp: str = str(p).zfill(8)
        for i in range(6, 14):
          self.outBuf[i] = ord(sf[i - 6])
        for i in range(15, 23):
          self.outBuf[i] = ord(sp[i - 15])
        self.calcCRC(self.outBuf)
        print('>>', self.outBuf)
        self.ser.write(self.outBuf)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="flowSensor.py",
        epilog="\nbe carefull and good lock !\n",
        formatter_class=lambda prog: argparse.HelpFormatter(prog, max_help_position=35)
    )
    parser.print_help()
    args = parser.parse_args()
    print()

    t = dummyFlowSensor()
    t.listen()
```


