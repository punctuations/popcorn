import webbrowser


# blueberry issue
def issue(args):
    """
    Submit issues about blueberry.

    :param args: arguments passed to command
    """
    webbrowser.open("https://github.com/punctuations/blueberry/issues/new")
