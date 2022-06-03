import json
import os


# strawberry help
def help(args):
    """
    Used to get more information about commands.

    :param args: arguments passed to command
    """

    f = open("./blame.json")
    blame = json.load(f)
    f.close()

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

    print(f"\n\u001b[38;5;1m {blame['name']} \u001b[0;0m| v\u001b[38;5;48m{blame['version']}\u001b[0;0m")
    print(f"\n\u001b[38;5;4m  💻 {blame['source']}\u001b[0;0m")
    print("\n\u001b[38;5;13m ### Description \033[0;0m\n")
    print(f"  {blame['description']}")
    print("\n\u001b[38;5;13m ### Commands \033[0;0m\n")
    for i, command in enumerate(commands):
        print(f"  \u001b[38;5;4m {command} \u001b[0;0m> {' '.join(definitions[i].split())}")