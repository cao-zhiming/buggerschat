from ftplib import error_temp
import socket

sk = socket.socket(type=socket.SOCK_DGRAM)
sk.bind(("", 1320))   # change this to your IP
# 30:黑
# 31:红
# 32:绿
# 33:黄
# 34:蓝色
# 35:紫色
# 36:深绿
# 37:白色
while 1:
    msg, addr1 = sk.recvfrom(1024)
    encoded = "\033[34m服务端：你好，USER1！你已经成功连接到BuggerChat-server@82.156.235.117:1320。\n\n一切正常。\033[0m".encode("utf-8")
    sk.sendto(encoded, addr1)
    alice_addr = addr1
    print("alice\n\n")
    print(alice_addr)
    print("\n\n")
    break
while 1:
    msg, addr2 = sk.recvfrom(1024)
    encoded = "\033[34m服务端：你好，USER2！你已经成功连接到BuggerChat-server@82.156.235.117:1320。\n\n一切正常。\033[0m".encode("utf-8")
    sk.sendto(encoded, addr2)
    bob_addr = addr2
    print("bob\n\n")
    print(bob_addr)
    print("\n\n")
    break
while 1:
    msg, addr = sk.recvfrom(1024)
    msg = msg.decode("utf-8")
    if "|" in msg:
        name, msg = msg.split("|", 1)
        to_send = "\033[32m" + name + ":" + msg + "\033[0m"
        to_send = to_send.encode("utf-8")
    error_msg = "\033[34m服务端：你好！你的地址已被服务端禁止访问。请联系曹智铭获取解决方法。\033[0m".encode("utf-8")
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


