import json
import os
import sys

from _utils import styled_print

DEV_DIR = f"{os.path.realpath(os.getcwd())}{os.sep}.strawberry"

f = open("./.berryrc")
config = json.load(f)
f.close()



# strawberry install
def install(args):
    berry_name = config["berry_name"]

    if not berry_name:
        styled_print.error("Please include a berry_name in config.")
        sys.exit(0)

    if DEV_DIR not in os.environ["PATH"]:
        if os.name == 'nt':
            # change path
            os.system(f"setx PATH '%PATH%{os.pathsep}{DEV_DIR}'")
            styled_print.success("Please run strawberry source to apply changes.")
        else:
            # edit .shellrc or .profile file
            rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"

            if os.path.exists(rcfile):
                shellrc = open(rcfile, "a")
                shellrc.write(f"\nexport PATH=$PATH{os.pathsep}{DEV_DIR}\n")
                shellrc.close()
            else:
                profile = open(f"{os.environ['HOME']}{os.sep}.profile", "a")
                profile.write(f"\nexport PATH=$PATH{os.pathsep}{DEV_DIR}\n")
                profile.close()

            styled_print.success("Please run strawberry source to apply changes.")
    else:
        styled_print.info("Berry already installed.")

