import os

from _utils import styled_print


def create_config(has_packed):
    with open('.kernelrc', "w") as config:
        if has_packed:
            config.write(f'{{\n"kernel_name": "{os.getcwd().split(os.sep)[-1]}",\n"kernel_type": "packed",'
                         f'\n"dev_cmd": "popcorn dev",\n"seed_cmd": "go build -o @dest",\n"advanced": {{'
                         f'\n"dev_node":  "-dev"\n}}\n}}')
        else:
            config.write(f'{{\n\t"kernel_name": "{os.getcwd().split(os.sep)[-1]}",\n\t"kernel_type": "unpacked",'
                         f'\n\t"unpacked_husk": "python @local/popcorn.py @args",\n\t"dev_cmd": "popcorn dev",'
                         f'\n\t"seed_cmd": "cp -r * @dest",\n\t"advanced": {{\n\t\t"dev_node":  "-dev"\n\t}}\n}}')
        config.close()


# popcorn init
# flags: --force: replaces any existing config with default; -p, --packed: generates the packed config
def init(args):
    """
    Used to initialize a .kernelrc configuration file.

    :param args: arguments passed to command
    """
    force = ['--force']
    packed = ['-p', '--packed']

    has_force_flag = [element for element in force if (element in args)]
    has_packed_flag = [element for element in packed if (element in args)]

    if not os.path.exists('.kernelrc'):
        create_config(has_packed_flag)
    elif has_force_flag:
        create_config(has_packed_flag)
    else:
        styled_print.info(".kernelrc already exists -- No changes made.")
