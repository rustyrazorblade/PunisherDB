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

    k3 = str("steve")
    v3 = str("smith")

    prepared = cap.prepare(key=k1, value=v1, timestamp=1, dependencies=[k2, k3])
    result = prepared.wait()
    print result

    cap.commit(1).wait()

    result = cap.get(k1).wait()
    print result

    result = cap.prepare(key=k1, value="is a lovely human", timestamp=2).wait()
    cap.commit(2)
    print "2 committed"

    print "Getting k1"
    result = cap.get(k1).wait()

    print result

    print "Getting k1 version"
    result = cap.getVersion(key=k1, timestamp=1).wait()
    print result

print "done"
