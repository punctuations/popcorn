def error(text):
    print(f"\u001b[38;5;1m error - \u001b[0;0m {text}")


def success(text):
    print(f"\u001b[38;5;48m success - \u001b[0;0m {text}")


def warning(text):
    print(f"\u001b[38;5;10m warning - \u001b[0;0m {text}")


def info(text):
    print(f"\u001b[38;5;4m info - \u001b[0;0m {text}")


def event(text):
    print(f"\u001b[38;5;13m event - \033[0;0m {text}")
