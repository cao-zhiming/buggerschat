import socket
import time
import os
sk=socket.socket(type=socket.SOCK_DGRAM)
server = ("82.156.235.117",1320)
print("BuggersChat软件\n可能是最适合程序员口味的轻量级聊天软件\n\n")

print("我们正在初始化。\n\n这期间，请你了解一些本软件的格式：\n白色字均是从本客户端输出的；蓝色字是从服务端发来的；绿色字是对方发来的；红色字是警告信息。\n\n")
name = input("现在，请你输入你希望使用的显示名称：")
name_e = name.encode("utf-8")
usr_to_write = "\n\n记录用户："+name+"\n\n"

sk.sendto(name_e,server)
time.sleep(0.1)
msg = sk.recv(1024)
msg = msg.decode("utf-8")
print("\n\n接下来，如果你希望发送内容，请输入send，空格，然后输入要发送的内容。\n\n")
print(msg)
while 1:
    u_command = input(">>>")
    comm,para=u_command.split(" ",1)
    if comm == "send":
        r_send = name+"|"+para
        r_send = r_send.encode("utf-8")
        sk.sendto(r_send,server)

    elif comm == "name":
        name = para
    elif comm == "req_ctrl":
        to_send = "usr|CTRL".encode("utf-8")
        sk.sendto(to_send,server)
    elif comm == "ctrl":
        to_send = "ctrl"+"|"+para
        to_send=to_send.encode("utf-8")
        sk.sendto(to_send,server)
    msg = sk.recv(1024)
    msg = msg.decode("utf-8")
    if ":" in msg:
        got_name,mes = msg.split(":",1)
        if mes == "BYE":
            break
        print(msg)
        r_file_now = "\n"+msg+"\n"

    else:
        print(msg)

time.sleep(1)
print("\n\n\n\n感谢你使用BuggersChat软件，本软件(C) Copyright 2022 曹智铭，https://czhiming.cn/\n\n")
time.sleep(3)
exit(0)
