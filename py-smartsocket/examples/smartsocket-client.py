#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from pysmartsocket import SmartSocketClient


if __name__ == "__main__":
    client = SmartSocketClient()
    print(client)

    client.connect("127.0.0.1:55333")
    print(client)

    client.switch_on()
    print(client)

    client.switch_off()
    print(client)
