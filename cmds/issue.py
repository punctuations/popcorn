import webbrowser


# popcorn issue
def issue(args):
    """
    Submit issues about popcorn.

    :param args: arguments passed to command
    """
    webbrowser.open("https://github.com/punctuations/popcorn/issues/new")
