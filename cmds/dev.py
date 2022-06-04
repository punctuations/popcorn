import json
import os
import shutil
import stat
import subprocess
import sys
import threading
import time
import re
from datetime import timedelta, datetime

from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer

from _utils import styled_print

listen = ["-l", "--listen"]

DEV_DIR = f"{os.path.realpath(os.getcwd())}{os.sep}.strawberry"

last_event = {"message": False}
event_delta = timedelta(seconds=int(2))

try:
    f = open("./.berryrc")
    config = json.load(f)
    f.close()
except FileNotFoundError:
    styled_print.error("Please create a .berryrc file.")
    sys.exit(0)

berry_name = config["berry_name"]
berry_type = config["berry_type"]

rcfile = f"{os.environ['HOME']}{os.sep}.{os.environ['SHELL'].split('/')[-1]}rc"
profile = f"{os.environ['HOME']}{os.sep}.profile"


def thread_compile():
    time.sleep(2)

    event_time_remaining = last_event["time"] + event_delta - datetime.now()
    if event_time_remaining.days <= -1:
        if last_event["message"]:
            styled_print.info("Updates to config detected, to see up-to-date changes re-run this command.")
        styled_print.event("Received compile event.")

        try:
            try:
                shutil.rmtree(f"{DEV_DIR}{os.sep}")
                os.mkdir(DEV_DIR)
            except FileExistsError:
                pass
            except FileNotFoundError:
                pass

            seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(f"{DEV_DIR}", config["seed_cmd"])
            exit_status = os.WEXITSTATUS(os.system(seed_cmd))

            # create unpacked berry.
            try:
                if berry_type.lower() == "unpacked":
                    try:
                        arg_stem = re.compile(re.escape("@args"), re.IGNORECASE).sub("\"$@\"", config["unpacked_stem"])
                        unpacked_stem = re.compile(re.escape("@local"), re.IGNORECASE).sub(DEV_DIR, arg_stem)
                        with open(f"{DEV_DIR}{os.sep}{berry_name}", "w") as berry:
                            berry.write(f"#!/bin/bash\n{unpacked_stem}")
                            berry.close()
                    except KeyError:
                        styled_print.error("Please include the unpacked_stem in the config.")
                        berry.close()
                        sys.exit(0)

            except KeyError:
                styled_print.error("Please include the berry_type in the config.")
                sys.exit(0)

            # edit permissions to all
            try:
                os.chmod(os.path.join(DEV_DIR, berry_name), stat.S_IRWXO | stat.S_IRWXU | stat.S_IRWXG)
            except FileNotFoundError:
                styled_print.error("Unable to amend permission of file; file not found.")
                sys.exit(0)

            if exit_status == 0:
                styled_print.success("Compiled successfully.")
            else:
                styled_print.error("Failed to compile, please check seed_cmd.")
        except KeyError:
            styled_print.error("Please enter in a seed_cmd")
            sys.exit(0)

        # rename berry to dev branch.
        try:
            dev_branch = config['advanced']['dev_branch']

            os.rename(f"{DEV_DIR}{os.sep}{berry_name}",
                      f"{DEV_DIR}{os.sep}{berry_name}{config['advanced']['dev_branch'] if dev_branch else '-dev'}")
        except FileNotFoundError:
            styled_print.error(f"Please have a file named {berry_name} as entry point.")
            sys.exit(0)


class Handler(FileSystemEventHandler):

    @staticmethod
    def on_any_event(event):
        if event.is_directory:
            return None

        try:
            p = subprocess.check_output(["git", "check-ignore", event.src_path])

            if type(p) is not bytes:
                ignored = False
            else:
                ignored = True
        except subprocess.CalledProcessError:
            ignored = False

        if event.src_path.split("/")[-1] == ".berryrc":
            last_event["message"] = True

        if not ignored:
            last_event["time"] = datetime.now()
            thread = threading.Thread(target=thread_compile)
            thread.start()


# strawberry dev
def dev(args):
    """
    Used to create development berries.

    :param args: arguments passed to command
    """
    has_listen_flag = [element for element in listen if (element in args)]
    listen_index = args.index(has_listen_flag[0]) if len(has_listen_flag) >= 1 else 0

    if not berry_name:
        styled_print.error("Please enter a berry_name in config.")
        sys.exit(0)

    # initialize development env
    shutil.rmtree(DEV_DIR, ignore_errors=True)
    os.mkdir(DEV_DIR)

    try:
        seed_cmd = re.compile(re.escape("@dest"), re.IGNORECASE).sub(DEV_DIR, config["seed_cmd"])
        exit_status = os.WEXITSTATUS(os.system(seed_cmd))

        # create packed berry.
        try:
            if berry_type.lower() == "unpacked":
                try:
                    arg_stem = re.compile(re.escape("@args"), re.IGNORECASE).sub("\"$@\"", config["unpacked_stem"])
                    unpacked_stem = re.compile(re.escape("@local"), re.IGNORECASE).sub(DEV_DIR, arg_stem)
                    with open(f"{DEV_DIR}{os.sep}{berry_name}", "w") as berry:
                        berry.write(f"#!/bin/bash\n{unpacked_stem}")
                        berry.close()
                except KeyError:
                    styled_print.error("Please include the unpacked_stem in the config.")
                    berry.close()
                    sys.exit(0)

        except KeyError:
            styled_print.error("Please include the berry_type in the config.")
            sys.exit(0)

        # edit permissions to all
        try:
            os.chmod(os.path.join(DEV_DIR, berry_name), stat.S_IRWXO | stat.S_IRWXU | stat.S_IRWXG)
        except FileNotFoundError:
            styled_print.error("Unable to amend permission of file; file not found.")
            sys.exit(0)

        if exit_status == 0:
            styled_print.success("Compiled successfully.")
        else:
            styled_print.error("Failed to compile, please check seed_cmd.")

    except KeyError:
        styled_print.error("Please enter in a seed_cmd")
        sys.exit(0)

    # rename berry to dev branch.
    try:
        dev_branch = config['advanced']['dev_branch']
        os.rename(f"{DEV_DIR}{os.sep}{berry_name}",
                  f"{DEV_DIR}{os.sep}{berry_name}{config['advanced']['dev_branch'] if dev_branch else '-dev'}")
    except FileNotFoundError:
        styled_print.error(f"Please have a file named {berry_name} as entry point.")
        sys.exit(0)

    if has_listen_flag:
        try:
            print(f"Listening to {args[listen_index + 1]}")
            watch_file = args[listen_index + 1]
        except IndexError:
            styled_print.error("Please specify a file or directory.")
            sys.exit(0)
    else:
        print("Listening to .")
        watch_file = "."

    if not os.path.exists(watch_file):
        styled_print.error("File or directory does not exist.")
        sys.exit(0)

    # Initialize logging event handler
    event_handler = Handler()

    # Initialize Observer
    observer = Observer()
    observer.schedule(event_handler, watch_file, recursive=True)

    # Start the observer
    observer.start()
    try:
        while True:
            # Set the thread sleep time
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
