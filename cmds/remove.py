import os
import subprocess

from _utils import styled_print


# strawberry remove
def remove(args):
    if len(args) >= 1:
        path = os.environ["PATH"]

        styled_print.info(f"Removing {args[0]}...")
        if f"{args[0]}{os.sep}.strawberry" in path:
            if os.name == 'nt':
                os.system(f"setx PATH '{f'{os.pathsep}'.join([i for i in path.split(os.pathsep) if f'{args[0]}{os.sep}.strawberry' not in i])}'")
                styled_print.success(f"Removed {args[0]}")
                styled_print.info(f"Please run strawberry source to apply changes.")
            else:
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

                styled_print.success(f"Removed {args[0]}")
                styled_print.info(f"To apply changes restart terminal.")
        else:
            styled_print.error("Berry not installed.")
    else:
        styled_print.error("Please specify a berry to remove.")
