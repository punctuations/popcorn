import os


# strawberry source
def source(args):
    if os.name == 'nt':
        os.system(". $profile")
    else:
        rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
        dotprofile = f"{os.environ['HOME']}{os.sep}.profile"

        if os.path.exists(rcfile):
            os.system(f"source {rcfile}")
        else:
            os.system(f"source {dotprofile}")
