from config import LANG

strings = {
    "zh-CN": {
        "str_intro": "BuggersChat软件\n可能是最适合程序员口味的轻量级聊天软件\n\n",
        "str_initializing": "我们正在初始化。\n\n这期间，请你了解一些本软件的格式：\n白色字均是从本客户端输出的；蓝色字是从服务端发来的；绿色字是对方发来的；红色字是警告信息。\n\n",
        "str_input_name": "现在，请你输入你希望使用的显示名称：",
        "str_recorded_user": "记录用户：",
        "str_hint_send": "接下来，如果你希望发送内容，请输入send，空格，然后输入要发送的内容。",
        "str_thanks_for_using": "感谢你使用BuggersChat软件，本软件(C) Copyright 2022 曹智铭，https://czhiming.cn/",
    },
    "en-US": {
        "str_intro": "Buggers Chat\n(May be the most programmer-styled lightweight chat program)\n\n",
        "str_initializing": "We were initializing the software. \n\nDuring the time, you may need to know the output format of this software: \nAll the white texts are printed in the local client. Blue texts are sent by the server side. Green texts are the message from who you are talking to. And the red texts are warnings. \n\n",
        "str_input_name": "Now, input the nickname you want to use: ",
        "str_recorded_user": "Recording user: ",
        "str_hint_send": "Now, if you want to send something, just input `send`, and a space, and then the content you want to send.",
        "str_thanks_for_using": "Thanks for using BuggersChat. This software (C) Copyright 2022 Zhiming Cao，https://czhiming.cn/",
    },
    "ja-JP": {
        "str_intro": "バッガーのチャット\nもしかしたら、一番プログラマー的な軽量チャットプログラムであるかもしれません\n\n",
        "str_initializing": "このプログラムは初期化しています。\n\nそれだけ、出力フォーマットが知ること必要です： \n白いテキストはクライエントの出力だ。ブルーなテキストはサーバーのメッセージです。 緑色なテキストは相手のメッセージです。赤いテキストは問題出力です。\n\n",
        "str_input_name": "では、ニックネームと入力してください：",
        "str_recorded_user": "ユーザーを記録している：",
        "str_hint_send": "もし、何が送信すること欲しいならば、「send」とスペースが一つと送信のメッセージと入力してください。",
        "str_thanks_for_using": "BuggersChatを利用してありがどうございました。このプログラムは：(C) Copyright 2022 Zhiming Cao，https://czhiming.cn/",
    },
}

def get_str(key):
    if LANG in strings.keys():
        return strings[LANG][key]
    else:
        return strings["en-US"][key]