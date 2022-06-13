import os


def create_config(has_packed):
    with open('.berryrc', "w") as config:
        if has_packed:
            config.write(f"{{\n'berry_name': '{os.getcwd().split(os.sep)[-1]}',\n'berry_type': 'packed',"
                         f"\n'dev_cmd': 'blueberry dev',\n'seed_cmd': 'go build -o @dest',\n'advanced': {{"
                         f"\n'dev_branch':  '-dev'\n}}\n}}")
        else:
            config.write(f"{{\n'berry_name': '{os.getcwd().split(os.sep)[-1]}',\n'berry_type': 'unpacked',"
                         f"\n'unpacked_stem': 'python @local/blueberry.py @args',\n'dev_cmd': 'blueberry dev',"
                         f"\n'seed_cmd': 'cp -r * @dest',\n'advanced': {{\n'dev_branch':  '-dev'\n}}\n}}")
        config.close()


# blueberry init
# flags: --force: replaces any existing config with default; -p, --packed: generates the packed config
def init(args):
    """
    Used to initialize a .berryrc configuration file.

    :param args: arguments passed to command
    """
    force = ['--force']
    packed = ['-p', '--packed']

    has_force_flag = [element for element in force if (element in args)]
    has_packed_flag = [element for element in packed if (element in args)]

    if not os.path.exists('.berryrc'):
        create_config(has_packed_flag)
    elif has_force_flag:
        create_config(has_packed_flag)
