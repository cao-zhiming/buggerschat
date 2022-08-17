use json::object;


pub fn get_string_by_language_and_key(language: &str, key: &str) -> String {
    let data = object! {
        "zh_CN": {
            "str_greeting": "欢迎来到BuggersChat，版本",
            "str_user_login": "%USERNAME%加入了聊天室。",
            "str_limit_reached": "抱歉，服务器已满，请联系服务器管理员。",
            "str_ask_username": "请输入用户名：",
            "str_ask_addr": "请输入要连接到的服务器的地址与端口号：",
            "str_suc_connect": "连接成功！",
            "str_fails_cnt": "无法连接到服务器：",
            "str_fails_login": "无法登录到服务器：",
            "str_disconnected": "连接已丢失。感谢您的使用！",
            "str_tech_detail": "这是一个技术细节，因此该错误信息并没有包含在语言本地化中。请联系开发者以寻求帮助。",
            "str_failed_to_send": "发送失败：",
        },
        "en_US": {
            "str_greeting": "Welcome to the BuggerChat, v",
            "str_user_login": "%USERNAME% joined the chat.",
            "str_limit_reached": "The people number limit of the server is reached. Please contact the administrator.",
            "str_ask_username": "Please input the username: ",
            "str_ask_addr": "Please input the IP address and the port number you want to connect to: ",
            "str_suc_connect": "Successfully connected! ",
            "str_fails_cnt": "Cannot connect to server: ",
            "str_fails_login": "Cannot login to server: ",
            "str_disconnected": "Connection lost. Thanks for using this software! ",
            "str_tech_detail": "",
            "str_failed_to_send": "Failed to send: ",
        },
    };
    if data.has_key(language) {
        let dic = &data[language];
        if dic.has_key(key) {
            dic[key].to_string()
        } else {
            key.to_string()
        }
    } else {
        let dic = &data["en_US"];
        if dic.has_key(key) {
            dic[key].to_string()
        } else {
            key.to_string()
        }
    }
}
