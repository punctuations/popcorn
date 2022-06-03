# strawberry init
import os


def init(args):
    force = ['--force']
    packed = ['-p', '--packed']

    has_force_flag = [element for element in force if (element in args)]
    has_packed_flag = [element for element in packed if (element in args)]

    if os.path.exists('.berryrc') and not has_force_flag:
        with open('.berryrc', "w") as config:
            if has_packed_flag:
                config.write(f"{{\n'berry_name': '{os.getcwd().split(os.sep)[-1]}',\n'berry_type': 'packed',\n'dev_cmd': 'strawberry dev',\n'seed_cmd': 'go build -o @dest',\n'advanced': {{\n'dev_branch':  '-dev'\n}}\n}}")
            else:
                config.write(f"{{\n'berry_name': '{os.getcwd().split(os.sep)[-1]}',\n'berry_type': 'unpacked',\n'unpacked_stem': 'python @local/strawberry.py @args',\n'dev_cmd': 'strawberry dev',\n'seed_cmd': 'cp -r * @dest',\n'advanced': {{\n'dev_branch':  '-dev'\n}}\n}}")
            config.close()
