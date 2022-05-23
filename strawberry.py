import os
import sys

alias = ["-a", "--alias"]
dev = ["-d", "--dev"]
debug = ["--debug"]


def load_cmds():
    cmds = os.listdir("cmds")  # get all files in /cmds
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

    if has_alias_flag:
        print("function berry () { eval $(strawberry $@); }")
    elif has_dev_flag:
        # * Run strawberry command based on .berryrc options and pass in the path
        print("READING FROM .berryrc")
        run_dev = getattr(cmds["dev"], "dev")
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
                run_build([command[0]])
        else:
            # command is: strawberry
            # * Run build command with .berryrc options
            print("READING FROM .berryrc")
            run_build = getattr(cmds["build"], "build")
            run_build(["."])

    if has_debug_flag:
        print(f"ran {command}: {len(command)}")


if __name__ == '__main__':
    strawberry(sys.argv[1:])
