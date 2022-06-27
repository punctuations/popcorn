import json
import os
import sys

from _utils import styled_print

DEV_DIR = f"{os.path.realpath(os.getcwd())}{os.sep}.blueberry"
PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.berries"

config = {}


def initialize_globals():
    global config

    try:
        f = open("./.berryrc")
        config = json.load(f)
        f.close()
    except FileNotFoundError:
        styled_print.error("Please create a .berryrc file.")
        sys.exit(0)


# blueberry install
# flags: -d, --dev: install development berries
def install(args):
    """
    Used to initialize environment for berries.

    :param args: arguments passed to command.
    """
    initialize_globals()

    dev = ["-d", "--dev"]
    has_dev_flag = [element for element in dev if (element in args)]

    berry_name = config["berry_name"]

    if not berry_name:
        styled_print.error("Please include a berry_name in config.")
        sys.exit(0)

    if not os.path.exists(f'{PROD_DIR}{os.sep}pits.sh'):
        if not os.path.exists(PROD_DIR):
            os.mkdir(PROD_DIR)

        with open(f'{PROD_DIR}{os.sep}pits.sh', "w") as pit:
            pit.write("#!/bin/bash\n")
            pit.close()

    if has_dev_flag:
        if DEV_DIR not in os.environ["PATH"]:
            if os.name == 'nt':
                # change path
                os.system(f"setx PATH '%PATH%{os.pathsep}{DEV_DIR}'")
                styled_print.success("Please run blueberry source to apply changes.")
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

            if os.name == 'nt':
                os.system(". $profile")
            else:
                rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
                dotprofile = f"{os.environ['HOME']}{os.sep}.profile"

                if os.path.exists(rcfile):
                    os.system(f"source {rcfile}")
                    styled_print.success("Please restart the terminal to apply changes.")
                else:
                    os.system(f"source {dotprofile}")
                    styled_print.success("Please restart the terminal to apply changes.")
        else:
            styled_print.info("Berry already installed.")
    else:
        rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
        dotprofile = f"{os.environ['HOME']}{os.sep}.profile"

        if os.path.exists(rcfile):
            shellrc = open(rcfile, "r+")
            if f". $HOME{os.sep}.berries{os.sep}pits.sh\n" not in shellrc.readlines():
                rclines = "\n".join(shellrc.readlines())
                shellrc.write(f"\n. $HOME{os.sep}.berries{os.sep}pits.sh\n{rclines}")
                shellrc.close()
            else:
                styled_print.info("Berry already installed.")
                shellrc.close()
        else:
            profile = open(dotprofile, "r+")
            if f". $HOME{os.sep}.berries{os.sep}pits.sh\n" not in profile.readlines():
                proflines = "\n".join(profile.readlines())
                profile.seek(0)
                profile.write(f"\n. $HOME{os.sep}.berries{os.sep}pits.sh\n{proflines}")
                profile.truncate()
                profile.close()
            else:
                styled_print.info("Berry already installed.")
                profile.close()
