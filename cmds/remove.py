import os
import shutil
import sys

from _utils import styled_print

PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.kernels"


# popcorn remove
# flags: -d, --dev: remove development kernels
def remove(args):
    """
    Used to remove both production kernels and development kernels.

    :param args: arguments passed to command
    """
    dev = ["-d", "--dev"]
    has_dev_flag = [element for element in dev if (element in args)]

    if len(args) >= 1:
        path = os.environ["PATH"]

        styled_print.info(f"Removing {args[0]}...")
        if has_dev_flag:
            if f"{args[0]}{os.sep}.popcorn" in path:
                if os.name == 'nt':
                    split_paths = path.split(os.pathsep)
                    removed_kernel = f'{args[0]}{os.sep}.popcorn'
                    fixed_path = f'{os.pathsep}'.join([i for i in split_paths if removed_kernel not in i])
                    os.system(f"[Environment]::SetEnvironmentVariable('PATH', '{fixed_path}', 'User')")
                    os.system(f"$env:PATH = '{fixed_path}'")
                else:
                    rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
                    profile = f"{os.environ['HOME']}{os.sep}.profile"

                    if os.path.exists(rcfile):
                        # remove form .shellrc
                        with open(rcfile, 'r+') as f:
                            data = ''.join([i for i in f if f"{args[0]}{os.sep}.popcorn" not in i])
                            f.seek(0)
                            f.write(data)
                            f.truncate()
                            f.close()
                    else:
                        # remove from .profile
                        with open(profile, 'r+') as f:
                            data = ''.join([i for i in f if f"{args[0]}{os.sep}.popcorn" not in i])
                            f.seek(0)
                            f.write(data)
                            f.truncate()
                            f.close()
                styled_print.success(f"Removed {args[0]}")
                styled_print.info("Please restart the terminal to apply changes.")
            else:
                styled_print.error("kernel not installed.")
        else:
            if f"{PROD_DIR}{os.sep}{args[0]}" in path:
                if os.name == 'nt':
                    split_paths = path.split(os.pathsep)
                    removed_kernel = f'{PROD_DIR}{os.sep}{args[0]}'
                    fixed_path = f'{os.pathsep}'.join([i for i in split_paths if removed_kernel not in i])
                    os.system(f"[Environment]::SetEnvironmentVariable('PATH', '{fixed_path}', 'User')")
                    os.system(f"$env:PATH = '{fixed_path}'")

                    try:
                        shutil.rmtree(f"{PROD_DIR}{os.sep}{args[0]}")
                    except FileNotFoundError:
                        styled_print.error("kernel not found.")
                        sys.exit(0)
                else:
                    if args[0] == "butter.sh":
                        styled_print.error("Not a kernel.")
                        sys.exit(0)

                    buttered = f'{PROD_DIR}{os.sep}butter.sh'
                    with open(buttered, 'r+') as butter:
                        data = ''.join([i for i in butter if args[0] not in i])
                        butter.seek(0)
                        butter.write(data)
                        butter.truncate()
                        butter.close()

                    try:
                        shutil.rmtree(f"{PROD_DIR}{os.sep}{args[0]}")
                    except FileNotFoundError:
                        styled_print.error("kernel not found.")
                        sys.exit(0)

                styled_print.success(f"Removed {args[0]}")
                styled_print.info("Please restart the terminal to apply changes.")
            else:
                styled_print.error("kernel not installed.")
    else:
        styled_print.error("Please specify a kernel to remove.")
