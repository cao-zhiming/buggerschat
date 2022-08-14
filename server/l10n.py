from config import LANG

strings = {
    "zh-CN": {
        "str_server_prefix": "服务端：",
        "str_user_greetings": "你好，USER1！你已经成功连接到一个",
        "str_user2_greetings": "你好，USER2！你已经成功连接到一个",
        "str_user_greetings_after": "。", # For some languages, there may be something after this.
        "str_normal": "一切正常。",
        "str_abandoned": "你好！你的地址已被服务端禁止访问。请联系曹智铭获取解决方法。",
    },
    "en-US": {
        "str_server_prefix": "Server side: ",
        "str_user_greetings": "Hello, USER1! You've been connected to a ",
        "str_user2_greetings": "Hello, USER2! You've been connected to a ",
        "str_user_greetings_after": ".", # For some languages, there may be something after this.
        "str_normal": "All works fine. ",
        "str_abandoned": "Hello! Your address has been abandoned from the server side. Please contact with Zhiming Cao for solutions. ",
    },
}

def get_str(key):
    if LANG in strings.keys():
        return strings[LANG][key]
    else:
        return strings["en-US"][key]
