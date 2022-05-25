import json
import os
import subprocess
import sys
import threading
import time
from datetime import timedelta, datetime

from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer

from _utils import styled_print

listen = ["-l", "--listen"]

DEV_DIR = f"{os.path.realpath(os.getcwd())}{os.sep}.strawberry"

last_event = {"message": False}
event_delta = timedelta(seconds=int(2))

f = open("./.berryrc")
config = json.load(f)
f.close()


def thread_compile():
    time.sleep(2)

    event_time_remaining = last_event["time"] + event_delta - datetime.now()
    if event_time_remaining.days <= -1:
        if last_event["message"]:
            styled_print.info("Updates to config detected, to see up-to-date changes re-run this command.")
        styled_print.event("Received compile event.")
        try:
            exit_status = os.WEXITSTATUS(os.system(config["seed_cmd"]))
            if exit_status == 0:
                styled_print.success("Compiled successfully.")
            else:
                styled_print.error("seed_cmd failed to compile.")
        except KeyError:
            styled_print.error("Please enter in a seed_cmd")


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
        except:
            ignored = False

        if event.src_path.split("/")[-1] == ".berryrc":
            last_event["message"] = True

        if not ignored:
            last_event["time"] = datetime.now()
            thread = threading.Thread(target=thread_compile)
            thread.start()


# strawberry dev
def dev(args):
    has_listen_flag = [element for element in listen if (element in args)]
    listen_index = args.index(has_listen_flag[0]) if len(has_listen_flag) >= 1 else 0

    # check if DEV_DIR exists, if not generate the dir using seed_cmd
    # initialize development env
    if not os.path.exists(DEV_DIR):
        styled_print.info("running seed_cmd")

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
