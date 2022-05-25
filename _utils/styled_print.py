def error(text):
    print(f"\033[1;37;41m ERR: \033[0;0m {text}")


def success(text):
    print(f"\033[2;32;49m success \033[0;0m {text}")


def warning(text):
    print(f"\033[2;33;49m warning - \033[0;0m {text}")


def info(text):
    print(f"\033[2;34;49m info - \033[0;0m {text}")


def event(text):
    print(f"\033[2;35;49m event - \033[0;0m {text}")
