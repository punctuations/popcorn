import os
import sys

listen = ["-l", "--listen"]

DEV_DIR = f"{os.path.realpath(os.getcwd())}{os.sep}.strawberry"


# strawberry dev
def dev(args):
    has_listen_flag = [element for element in listen if (element in args)]
    listen_index = args.index(has_listen_flag[0]) if len(has_listen_flag) >= 1 else 0

    # check if DEV_DIR exists, if not generate the dir using seed_cmd
    # initialize development env
    if not os.path.exists(DEV_DIR):
        print("running seed_cmd")

    if has_listen_flag:
        try:
            print(f"Listening to {args[listen_index + 1]}")
            watch_file = args[listen_index + 1]
        except IndexError:
            print("Please specify a file or directory.")
            sys.exit(0)
    else:
        print("Listening to .")
        watch_file = "."

    if not os.path.exists(watch_file):
        print("File or directory does not exist.")
        sys.exit(0)

    if os.path.isfile(watch_file):
        # watch_file is file, watch only the file.
        while True:
            try:
                if f"CHANGE IN FILE {watch_file}":
                    print("LISTENING FOR CHANGES")
            except KeyboardInterrupt:
                sys.exit(0)
    else:
        # watch_file is directory, watch for changes to files in directory -- exclude .gitignore files?
        while True:
            try:
                if f"CHANGE IN DIRECTORY {watch_file}":
                    print("LISTENING FOR CHANGES")
            except KeyboardInterrupt:
                sys.exit(0)
