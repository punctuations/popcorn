import json
import os
import re
import shutil
import stat
import sys
import threading

from _utils import styled_print

config = {}
kernel_name = ""
kernel_type = ""


def initialize_globals():
    global kernel_name
    global kernel_type
    global config
    try:
        f = open("./.kernelrc")
        config = json.load(f)
        f.close()
    except FileNotFoundError:
        styled_print.error("Please create a .kernelrc file.")
        sys.exit(0)

    kernel_name = config["kernel_name"]
    kernel_type = config["kernel_type"]


PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.kernels"

rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
dotprofile = f"{os.environ['HOME']}{os.sep}.profile"


def build_thread(output):
    path_init = False

    try:
        shutil.rmtree(f"{PROD_DIR}{os.sep}{output}")
        os.mkdir(PROD_DIR)
        if kernel_type.lower() == "unpacked":
            os.mkdir(f"{PROD_DIR}{os.sep}{output}")
    except FileExistsError:
        pass
    except FileNotFoundError:
        pass
    except KeyError:
        styled_print.error("Please include a kernel_name in the config.")
        sys.exit(0)

    try:
        # run seed_cmd
        if kernel_type.lower() == "unpacked":
            try:
                os.mkdir(f"{PROD_DIR}{os.sep}{output}")
            except FileExistsError:
                pass
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE) \
                .sub(f"{PROD_DIR}{os.sep}{output}", config["seed_cmd"])
        else:
            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(f"{PROD_DIR}", config["seed_cmd"])
        exit_status = os.WEXITSTATUS(os.system(seed_cmd))

        # create unpacked kernel with unpacked stem.
        if kernel_type.lower() == "unpacked":
            try:
                arg_stem = re.compile(re.escape("@args"), re.IGNORECASE).sub("\"$@\"", config["unpacked_husk"])
                unpacked_husk = re.compile(re.escape("@local"), re.IGNORECASE) \
                    .sub(f"{PROD_DIR}{os.sep}{output[:-1] if output.endswith('/') else output}", arg_stem)
                with open(f"{PROD_DIR}{os.sep}{output}{kernel_name}", "w") as kernel:
                    kernel.write(f"#!/bin/bash\n{unpacked_husk}")
                    kernel.close()
            except KeyError:
                styled_print.error("Please include the unpacked_husk in the config.")
                kernel.close()
                sys.exit(0)

        # change permissions
        try:
            os.chmod(os.path.join(PROD_DIR, output if kernel_type.lower() == "packed" else f"{output}{kernel_name}"),
                     stat.S_IRWXO | stat.S_IRWXU | stat.S_IRWXG)
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
            buttered = open(f'{PROD_DIR}{os.sep}butter.sh', 'r')
            lines = buttered.readlines()
            buttered.close()

            if kernel_type == "packed":
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}butter.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                        builds.close()
            else:
                if f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n" not in lines:
                    with open(f'{PROD_DIR}{os.sep}butter.sh', "a") as builds:
                        builds.write(f"export PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                        builds.close()
        except FileNotFoundError:
            with open(f'{PROD_DIR}{os.sep}butter.sh', "w") as butter:
                if kernel_type == "packed":
                    butter.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}\n")
                else:
                    butter.write(f"#!/bin/bash\nexport PATH=$PATH{os.pathsep}{PROD_DIR}{os.sep}{output}\n")
                butter.close()
    else:
        if PROD_DIR not in os.environ["PATH"] and kernel_type.lower() == "packed":
            path_init = True
            os.system(f"[Environment]::SetEnvironmentVariable('PATH', '$env:PATH{os.pathsep}{PROD_DIR}', 'User')")
            os.system(f"$env:PATH += '{os.pathsep}{PROD_DIR}'")
            styled_print.success("Added to path")

        if f"{PROD_DIR}{os.sep}{output}" not in os.environ["PATH"] and kernel_type.lower() == "unpacked":
            path_init = True
            os.system(
                f"[Environment]::SetEnvironmentVariable('PATH', '$env:PATH{os.pathsep}{PROD_DIR}{output}', 'User')")
            os.system(f"$env:PATH += '{os.pathsep}{PROD_DIR}{output}'")
            styled_print.success("Added to path")

    if os.name != "nt":
        if os.path.exists(rcfile):
            shellrc = open(rcfile, "r")
            rclines = shellrc.read()
            shellrc.close()
            if f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n" not in rclines:
                with open(rcfile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n")
                    init.write(rclines)
                    init.close()
        else:
            profile = open(dotprofile, "r")
            proflines = profile.read()
            profile.close()
            if f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n" not in proflines:
                with open(dotprofile, "w") as init:
                    init.write(f". \"$HOME{os.sep}.kernels{os.sep}butter.sh\"\n")
                    init.write(proflines)
                    init.close()
        if path_init:
            if os.name == 'nt':
                os.system(". $profile")
            else:
                if os.path.exists(rcfile):
                    os.system(f"source {rcfile}")
                else:
                    os.system(f"source {dotprofile}")

    styled_print.success(f"Successfully built {kernel_name}.")


# popcorn build
# flags: -o, --output: change the output directory of the kernel
def build(args):
    """
    Used to create production-level kernels.

    :param args: arguments passed to command
    """
    initialize_globals()

    output = ["-o", "--output"]
    # when file is placed here make sure to set path variable to file containing accessible target
    # ex. (unpacked) export PATH = $PATH:$HOME/.local/bin/popcorn
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
            output = kernel_name
        except KeyError:
            styled_print.error("Please include a kernel_name in the config.")
            sys.exit(0)

    try:
        if kernel_type.lower() == "unpacked":
            output = output + os.sep
    except KeyError:
        styled_print.error("Please include the kernel_type in the config.")
        sys.exit(0)

    thread = threading.Thread(target=build_thread, args=(output,))
    thread.start()
