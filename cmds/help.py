import json
import os
import sys

from _utils import styled_print


# blueberry help
def help(args):
    """
    Used to get more information about commands.

    :param args: arguments passed to command
    """

    ROOT_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))

    def resource_path(relative_path):
        """ Get absolute path to resource, works for dev and for PyInstaller """
        try:
            # PyInstaller creates a temp folder and stores path in _MEIPASS
            base_path = sys._MEIPASS
        except Exception:
            base_path = ROOT_DIR

        return os.path.join(base_path, relative_path)

    f = open(resource_path("blame.json"))
    blame = json.load(f)
    f.close()

    if len(args) < 1:
        command_list = os.listdir(path=os.path.dirname(os.path.realpath(__file__)))

        commands = []
        definitions = []

        for file in command_list:
            if not file.startswith("_"):
                commands.append(file.split(".")[0])

        for file in commands:
            with open(f"{os.path.dirname(os.path.realpath(__file__))}{os.sep}{file}.py", "r") as f:
                contents = f.read()
                definitions.append(contents.split("\"\"\"")[1].split(":")[0])
                f.close()

        print(f"\n \u001b[40;1;1m {blame['name']} \u001b[0;0m v\u001b[38;5;48m{blame['version']}\u001b[0;0m")
        print(f"\n\u001b[38;5;8m  💻 {blame['source']}\u001b[0;0m")
        print("\n\u001b[38;5;13m ### Description \033[0;0m\n")
        print(f"  {blame['description']}")
        print("\n\u001b[38;5;13m ### Commands \033[0;0m\n")
        for i, command in enumerate(commands):
            print(f"  • \u001b[38;5;4m {command} \u001b[0;0m> {' '.join(definitions[i].split())}")
    else:
        command_file = f"{os.path.dirname(os.path.realpath(__file__))}{os.sep}{args[0]}.py"
        if os.path.exists(command_file):
            command = open(command_file, "r")
            contents = command.read()
            command.close()
            command_desc = contents.split('\"\"\"\n')[1].split("\n")[0]

            print(f"\n \u001b[40;5;1m {blame['name']} \u001b[0;0m v\u001b[38;5;48m{blame['version']}\u001b[0;0m")
            print(f"\n\u001b[38;5;8m  💻 {blame['source']}\u001b[0;0m")
            print("\n\u001b[38;5;13m ### Usage \033[0;0m\n")
            print(f"  {command_desc}")
            print("\n\u001b[38;5;13m ### Flags \033[0;0m\n")
            for flags in contents.split("# flags: ")[1].split("\n")[0].split(";"):
                flag = flags.split(":")[0]
                flag_desc = flags.split(":")[1].split("\n")[0]
                print(f"  • \u001b[38;5;4m {flag} \u001b[0;0m> {flag_desc}")
        else:
            styled_print.error("Command not found")
