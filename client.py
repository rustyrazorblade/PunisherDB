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

    k1 = str("jon")
    v1 = str("haddad")

    k2 = str("dave")
    v2 = str("haddad")

    prepared = cap.prepare(key=k1, value=v1, timestamp=1)
    result = prepared.wait()
    print result

    cap.commit(1).wait()

    result = cap.get(k1).wait()
    print result

print "done"
