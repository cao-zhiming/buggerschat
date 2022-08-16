from ftplib import error_temp
from l10n import get_str
import socket

sk = socket.socket(type=socket.SOCK_DGRAM)

# Change this to your IP.
sk.bind(("0.0.0.0", 1320))

while True:
    msg, addr1 = sk.recvfrom(1024)
    encoded = "\033[34m{0}{1}BuggerChat-server{2}\n\n{3}\033[0m".format(get_str("str_server_prefix"), get_str("str_user_greetings"), get_str("str_user_greetings_after"), get_str("str_normal")).encode("utf-8")
    sk.sendto(encoded, addr1)
    alice_addr = addr1
    print("alice\n\n")
    print(alice_addr)
    print("\n\n")
    break
while True:
    msg, addr2 = sk.recvfrom(1024)
    encoded = "\033[34m{0}{1}你已经成功连接到BuggerChat-server{2}\n\n{3}\033[0m".format(get_str("str_server_prefix"), get_str("str_user2_greetings"), get_str("str_user_greetings_after"), get_str("str_normal")).encode("utf-8")
    sk.sendto(encoded, addr2)
    bob_addr = addr2
    print("bob\n\n")
    print(bob_addr)
    print("\n\n")
    break
while True:
    msg, addr = sk.recvfrom(1024)
    msg = msg.decode("utf-8")
    if "|" in msg:
        name, msg = msg.split("|", 1)
        to_send = "\033[32m" + name + ":" + msg + "\033[0m"
        to_send = to_send.encode("utf-8")
    error_msg = "\033[34m{0}{1}\033[0m".format(get_str("str_server_prefix"), get_str("str_abandoned")).encode("utf-8")
    if addr == alice_addr:
        sk.sendto(to_send, bob_addr)
        sk.sendto(to_send, tmp_addr)
    elif addr == bob_addr:
        sk.sendto(to_send, alice_addr)
        sk.sendto(to_send, tmp_addr)
    elif addr == tmp_addr:
        sk.sendto(to_send, alice_addr)
        sk.sendto(to_send, bob_addr)
    else:
        tmp_addr = addr
        sk.sendto(to_send, alice_addr)
        sk.sendto(to_send,bob_addr)
        sk.sendto(to_send,addr)


