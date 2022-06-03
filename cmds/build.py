import json
import os
import re
import shutil
import stat
import sys
import threading

from _utils import styled_print

try:
    f = open("./.berryrc")
    config = json.load(f)
    f.close()
except FileNotFoundError:
    styled_print.error("Please create a .berryrc file.")
    sys.exit(0)

PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.berries"
berry_name = config["berry_name"]
berry_type = config["berry_type"]

rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
dotprofile = f"{os.environ['HOME']}{os.sep}.profile"


def build_thread(output):
    try:
        shutil.rmtree(f"{PROD_DIR}{os.sep}{output}")
        os.mkdir(PROD_DIR)
        if berry_type.lower() == "unpacked":
            os.mkdir(f"{PROD_DIR}{os.sep}{output}")
    except FileExistsError:
        pass
    except FileNotFoundError:
        pass
    except KeyError:
        styled_print.error("Please include a berry_name in the config.")
        sys.exit(0)

    try:
        if berry_type.lower() == "unpacked":
            try:
                os.mkdir(f"{PROD_DIR}{os.sep}{output}")
            except FileExistsError:
                pass
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(f"{PROD_DIR}{os.sep}{output}", config["seed_cmd"])
        else:
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(f"{PROD_DIR}", config["seed_cmd"])
        exit_status = os.WEXITSTATUS(os.system(seed_cmd))

        # create unpacked berry.
        if berry_type.lower() == "unpacked":
            try:
                arg_stem = re.compile(re.escape("@args"), re.IGNORECASE).sub(f"\"$@\"", config["unpacked_stem"])
                unpacked_stem = re.compile(re.escape("@local"), re.IGNORECASE).sub(f"{PROD_DIR}{os.sep}{output[:-1] if output.endswith('/') else output}", arg_stem)
                with open(f"{PROD_DIR}{os.sep}{output}{berry_name}", "w") as berry:
                    berry.write(f"#!/bin/bash\n{unpacked_stem}")
                    berry.close()
            except KeyError:
                styled_print.error("Please include the unpacked_stem in the config.")
                berry.close()
                sys.exit(0)

        try:
            os.chmod(os.path.join(PROD_DIR, output if berry_type.lower() == "packed" else f"{output}{berry_name}"), stat.S_IRWXO | stat.S_IRWXU | stat.S_IRWXG)
        except FileNotFoundError:
            styled_print.error("Unable to amend permission of file; file not found.")
            sys.exit(0)

        if exit_status == 0:
            styled_print.success("Compiled successfully!")

    except KeyError:
        styled_print.error("Please enter in a seed_cmd")
        sys.exit(0)

    if os.name != "nt":
        try:
            pits = open(f'{PROD_DIR}{os.sep}pits.sh', 'r')
            lines = pits.readlines()
            pits.close()

            if berry_type == "packed":
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}pits.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                        builds.close()
            else:
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}pits.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                        builds.close()
        except FileNotFoundError:
            with open(f'{PROD_DIR}{os.sep}pits.sh', "w") as pit:
                if berry_type == "packed":
                    pit.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                else:
                    pit.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                pit.close()
    else:
        if PROD_DIR not in os.environ["PATH"] and berry_type.lower() == "packed":
            os.system(f"setx PATH '%PATH%{os.pathsep}{PROD_DIR}'")

        if f"{PROD_DIR}{os.sep}{output}" not in os.environ["PATH"] and berry_type.lower() == "unpacked":
            os.system(f"setx PATH '%PATH%{os.pathsep}{PROD_DIR}{output}'")

    styled_print.success("Added to path")

    if os.name != "nt":
        if os.path.exists(rcfile):
            shellrc = open(rcfile, "r")
            rclines = shellrc.read()
            shellrc.close()
            if f". \"$HOME{os.sep}.berries{os.sep}pits.sh\"\n" not in rclines:
                with open(rcfile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.berries{os.sep}pits.sh\"\n")
                    init.write(rclines)
                    init.close()
        else:
            profile = open(dotprofile, "r")
            proflines = profile.read()
            profile.close()
            if f". \"$HOME{os.sep}.berries{os.sep}pits.sh\"\n" not in proflines:
                with open(dotprofile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.berries{os.sep}pits.sh\"\n")
                    init.write(proflines)
                    init.close()
        if os.name == 'nt':
            os.system(". $profile")
        else:
            if os.path.exists(rcfile):
                os.system(f"source {rcfile}")
            else:
                os.system(f"source {dotprofile}")

    styled_print.success("Please restart the terminal session to apply changes.")


# strawberry build
def build(args):
    output = ["-o", "--output"]
    # when file is placed here make sure to set path variable to file containing accessible target
    # ex. (unpacked) export PATH = $PATH:$HOME/.local/bin/strawberry
    # ex. (packed) export PATH  = $PATH:$HOME/.local/bin
    has_output_flag = [element for element in output if (element in args)]
    output_index = args.index(has_output_flag[0]) if len(has_output_flag) >= 1 else 0

    if not os.path.exists(PROD_DIR):
        os.mkdir(PROD_DIR)

    if has_output_flag:
        try:
            output = args[output_index + 1]
        except IndexError:
            styled_print.error("Please specify an output name.")
            sys.exit(0)
    else:
        try:
            output = berry_name
        except KeyError:
            styled_print.error("Please include a berry_name in the config.")
            sys.exit(0)

    try:
        if berry_type.lower() == "unpacked":
            output = output + os.sep
    except KeyError:
        styled_print.error("Please include the berry_type in the config.")
        sys.exit(0)

    thread = threading.Thread(target=build_thread, args=(output,))
    thread.start()
