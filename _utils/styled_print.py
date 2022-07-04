def error(text):
    print(f"[38;5;1m error - [0;0m {text}")


def success(text):
    print(f"[38;5;48m success - [0;0m {text}")


def warning(text):
    print(f"[38;5;10m warning - [0;0m {text}")


def info(text):
    print(f"[38;5;4m info - [0;0m {text}")


def event(text):
    print(f"[38;5;13m event - [0;0m {text}")


def debug(text):
    print(f"[38;5;37m debug - [0;0m {text}")
