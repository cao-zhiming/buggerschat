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
    "ja-JP": {
        "str_server_prefix": "サーバーサイド：",
        "str_user_greetings": "こんにちは、USER１！お客様は",
        "str_user2_greetings": "こんにちは、USER２！お客様は",
        "str_user_greetings_after": "を一つ接続しました。", # For some languages, there may be something after this.
        "str_normal": "全て正常です。",
        "str_abandoned": "すみません！このアドレスはサーバーサイドから禁制しました。曹智銘さん（そう・ちめい）を連絡して解き方を求める。",
    },
}

def get_str(key):
    if LANG in strings.keys():
        return strings[LANG][key]
    else:
        return strings["en-US"][key]
