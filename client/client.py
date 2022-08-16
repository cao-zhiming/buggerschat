import socket
import time
import os
from config import SERVER_IP
from ..server.l10n import get_str
sk=socket.socket(type=socket.SOCK_DGRAM)

# Connect to the server
server = (SERVER_IP, 1320)
print(get_str("str_intro"))

print(get_str("str_initializing"))
name = input(get_str("str_input_name"))
name_e = name.encode("utf-8")
usr_to_write = "\n\n{0}".format(get_str("str_recorded_user"))+name+"\n\n"

sk.sendto(name_e,server)
time.sleep(0.1)
msg = sk.recv(1024)
msg = msg.decode("utf-8")
print("\n\n{0}\n\n".format(get_str("str_hint_send")))
print(msg)

while True:
    # Accept user commands and response.
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
print("\n\n\n\n{0}\n\n".format(get_str("str_thanks_for_using")))
time.sleep(3)
exit(0)
