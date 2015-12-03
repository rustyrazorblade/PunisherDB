import socket
import string
import random
sock = socket.create_connection(("127.0.0.1", "6000"))

def eval(c):
    sock.sendall(c + "\n")
    resp = sock.recv(1024)
    return resp

def key():
    return "".join([random.choice(string.ascii_lowercase) for _ in range(10)])

for x in range(10000):
    k1 = key()
    print eval("prepare {} haddad {} steve,jim".format(k1, x))
    print eval("commit {}".format(x))

# print eval("prepare steve whatever 1 jim,jon")
# print eval("get jon")
