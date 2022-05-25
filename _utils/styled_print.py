def error(text):
    print(f"\033[1;37;41m ERR: \033[0;0m {text}")


def success(text):
    print(f"\033[1;30;42m SUCCESS: \033[0;0m {text}")


def warning(text):
    print(f"\033[1;31;43m WARNING: \033[0;0m {text}")


def info(text):
    print(f"\033[1;37;44m INFO: \033[0;0m {text}")
