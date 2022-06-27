def error(text):
    print(f"\033[38;5;1m error - \033[0;0m {text}")


def success(text):
    print(f"\033[38;5;48m success - \033[0;0m {text}")


def warning(text):
    print(f"\033[38;5;10m warning - \033[0;0m {text}")


def info(text):
    print(f"\033[38;5;4m info - \033[0;0m {text}")


def event(text):
    print(f"\033[38;5;13m event - \033[0;0m {text}")


def debug(text):
    print(f"\033[38;5;37m debug - \033[0;0m {text}")
