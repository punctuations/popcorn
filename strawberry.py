import json
import os
import sys

from _utils import styled_print

alias = ["-a", "--alias"]
dev = ["-d", "--dev"]
debug = ["--debug"]

ROOT_DIR = os.path.dirname(os.path.realpath(__file__))


def load_cmds():
    cmds = os.listdir(f"{ROOT_DIR}{os.sep}cmds")  # get all files in /cmds
    cmds = [s for s in cmds if s[0] not in "_."]  # remove hidden files
    cmds = [s[:-3] for s in cmds]  # remove '.py'
    cmds_dict = {}  # init cmds_dict

    for s in cmds:
        cmds_dict[s] = __import__("cmds" + "." + s, fromlist=["*"])

    # sort by key
    return cmds_dict


def strawberry(command):
    has_alias_flag = [element for element in alias if (element in command)]
    # get positional index where it is in list
    has_dev_flag = [element for element in dev if (element in command)]
    dev_index = command.index(has_dev_flag[0]) if len(has_dev_flag) >= 1 else 0
    has_debug_flag = [element for element in debug if (element in command)]
    cmds = load_cmds()

    f = open("./.berryrc")
    config = json.load(f)
    f.close()

    if has_debug_flag:
        styled_print.info(f"ran {command}: {len(command)}")

    if has_alias_flag:
        print("function berry () { eval $(strawberry $@); }")
    elif has_dev_flag:
        # * Run strawberry command based on .berryrc options and pass in the path
        run_dev = getattr(cmds["dev"], "dev")
        try:
            dev_command = config["dev_cmd"]
            os.system(dev_command)
        except KeyError:
            run_dev(command[dev_index + 1:] if len(command) >= 2 else ["-l", "."])
    else:
        # if argument is passed in
        if len(command) >= 1:
            # if command does not exist go to build command.
            try:
                run_command = getattr(cmds[command[0]], command[0])
                run_command(command[1:])
            except KeyError:
                # command is: strawberry /path/to/files
                run_build = getattr(cmds["build"], "build")
                run_build(command)
        else:
            # command is: strawberry
            # * Run install command
            run_install = getattr(cmds["install"], "install")
            run_install([])


if __name__ == '__main__':
    strawberry(sys.argv[1:])
