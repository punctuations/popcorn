import os
import shutil
import sys

from _utils import styled_print

PROD_DIR = f"{os.path.expanduser('~')}{os.sep}.berries"


# strawberry remove
# flags: -d, --dev: remove development berries
def remove(args):
    """
    Used to remove both production berries and development berries.

    :param args: arguments passed to command
    """
    dev = ["-d", "--dev"]
    has_dev_flag = [element for element in dev if (element in args)]

    if len(args) >= 1:
        path = os.environ["PATH"]

        styled_print.info(f"Removing {args[0]}...")
        if f"{args[0]}{os.sep}.strawberry" in path:
            if os.name == 'nt':
                split_paths = path.split(os.pathsep)
                removed_berry = f'{args[0]}{os.sep}.strawberry'
                os.system(
                    f"setx PATH '{f'{os.pathsep}'.join([i for i in split_paths if removed_berry not in i])}'")
            else:
                if has_dev_flag:
                    rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
                    profile = f"{os.environ['HOME']}{os.sep}.profile"

                    if os.path.exists(rcfile):
                        # remove form .shellrc
                        with open(rcfile, 'r+') as f:
                            data = ''.join([i for i in f if f"{args[0]}{os.sep}.strawberry" not in i])
                            f.seek(0)
                            f.write(data)
                            f.truncate()
                            f.close()
                    else:
                        # remove from .profile
                        with open(profile, 'r+') as f:
                            data = ''.join([i for i in f if f"{args[0]}{os.sep}.strawberry" not in i])
                            f.seek(0)
                            f.write(data)
                            f.truncate()
                            f.close()
                else:
                    if args[0] == "pits.sh":
                        styled_print.error("Not a berry.")
                        sys.exit(0)

                    pits = f'{PROD_DIR}{os.sep}pits.sh'
                    with open(pits) as pit:
                        data = ''.join([i for i in pit if args[0] not in i])
                        pit.seek(0)
                        pit.write(data)
                        pit.truncate()
                        pit.close()

                    try:
                        shutil.rmtree(f"{PROD_DIR}{os.sep}{args[0]}")
                    except FileNotFoundError:
                        styled_print.error("Berry not found.")
                        sys.exit(0)
            styled_print.success(f"Removed {args[0]}")
            styled_print.info("Please restart the terminal to apply changes.")
        else:
            styled_print.error("Berry not installed.")
    else:
        styled_print.error("Please specify a berry to remove.")
