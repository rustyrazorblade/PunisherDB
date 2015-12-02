import sys
sys.path.append("")

import capnp
import ramp_capnp
import socket

from uuid import uuid4
from time import time

if __name__  == "__main__":
    client = capnp.TwoPartyClient("127.0.0.1:6000")
    cap = client.bootstrap()
    cap = cap.cast_as(ramp_capnp.RampInterface)


print "done"
