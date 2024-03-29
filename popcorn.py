import json
import os
import sys

from _utils import styled_print

alias = ["-a", "--alias"]
dev = ["-d", "--dev"]
debug = ["--debug"]
help_flag = ["--help", "-h"]
version_flag = ["--version", "-v"]

ROOT_DIR = os.path.dirname(os.path.realpath(__file__))


def resource_path(relative_path):
    """ Get absolute path to resource, works for dev and for PyInstaller """
    try:
        # PyInstaller creates a temp folder and stores path in _MEIPASS
        base_path = sys._MEIPASS
    except Exception:
        base_path = ROOT_DIR

    return os.path.join(base_path, relative_path)


def load_cmds(has_debug_flag: list[str]):
    styled_print.info("begin load") if has_debug_flag else None
    cmds = os.listdir(resource_path("cmds"))  # get all files in /cmds
    cmds = [s for s in cmds if s[0] not in "_."]  # remove hidden files
    cmds = [s[:-3] for s in cmds]  # remove '.py'
    cmds_dict = {}  # init cmds_dict

    styled_print.info("before command load loop") if has_debug_flag else None
    for s in cmds:
        cmds_dict[s] = __import__("cmds" + "." + s, fromlist=["*"])
        styled_print.info(f"imported {s}") if has_debug_flag else None

    # sort by key
    styled_print.info("return cmds") if has_debug_flag else None
    return cmds_dict


def popcorn(command):
    has_alias_flag = [element for element in alias if (element in command)]
    # get positional index where it is in list
    has_dev_flag = [element for element in dev if (element in command)]
    dev_index = command.index(has_dev_flag[0]) if len(has_dev_flag) >= 1 else 0
    has_debug_flag = [element for element in debug if (element in command)]
    debug_index = command.index(has_debug_flag[0]) if len(has_debug_flag) >= 1 else 0
    has_help_flag = [element for element in help_flag if (element in command)]
    help_index = command.index(has_help_flag[0]) if len(has_help_flag) >= 1 else 0
    has_version_flag = [element for element in version_flag if (element in command)]

    styled_print.info("before load") if has_debug_flag else None
    cmds = load_cmds(has_debug_flag)
    styled_print.info("after load") if has_debug_flag else None

    cmd_with_debug = 0 if not has_debug_flag or debug_index != 0 else debug_index + 1

    styled_print.debug(f"ran with args {command}: {len(command)}") if has_debug_flag else None

    if has_alias_flag:
        styled_print.debug("running alias command") if has_debug_flag else None

        print("function pop () { eval $(popcorn $@); }")
    elif has_version_flag:
        f = open(resource_path("blame.json"))
        blame = json.load(f)
        f.close()
        styled_print.info(blame["version"])
    elif has_dev_flag and len(command) == 1:
        styled_print.debug("running dev command from flag") if has_debug_flag else None
        # * Run popcorn command based on .kernelrc options and pass in the path

        try:
            f = open("./.kernelrc")
            config = json.load(f)
            f.close()
        except FileNotFoundError:
            styled_print.error("Please create a .kernelrc file.")
            sys.exit(0)

        run_dev = getattr(cmds["dev"], "dev")
        try:
            dev_command = config["dev_cmd"]
            os.system(dev_command)
        except KeyError:
            run_dev(command[dev_index + 1:] if len(command) >= 2 else ["-l", "."])
    elif has_help_flag:
        styled_print.debug("running help command from flag") if has_debug_flag else None

        run_help = getattr(cmds["help"], "help")
        run_help(command[help_index + 1:])
    elif command[cmd_with_debug] == 'theater':
        # alias for theatre
        styled_print.debug("running theatre command") if has_debug_flag else None

        run_install = getattr(cmds["theatre"], "theatre")
        run_install(command[cmd_with_debug + 1:] if command[cmd_with_debug + 1:] != ['--debug'] else [])
    elif command[cmd_with_debug] == 'uninstall':
        # alias for theatre
        styled_print.debug("running remove command") if has_debug_flag else None

        run_install = getattr(cmds["remove"], "remove")
        run_install(command[cmd_with_debug + 1:] if command[cmd_with_debug + 1:] != ['--debug'] else [])
    else:
        # if argument is passed in
        if len(command) >= 1:
            # if command does not exist go to build command.
            try:
                run_command = getattr(cmds[command[cmd_with_debug]], command[cmd_with_debug])

                styled_print.debug(f"running {command[cmd_with_debug]} command") if has_debug_flag else None
                run_command(command[cmd_with_debug + 1:] if command[cmd_with_debug + 1:] != ['--debug'] else [])
            except KeyError:
                styled_print.debug("running build command as fallback") if has_debug_flag else None

                # command is: popcorn /path/to/files
                run_build = getattr(cmds["build"], "build")
                run_build(command)
        else:
            styled_print.debug("running install command from no args") if has_debug_flag else None

            # command is: popcorn
            # * Run install command
            run_install = getattr(cmds["install"], "install")
            run_install([])


if __name__ == '__main__':
    popcorn(sys.argv[1:])
