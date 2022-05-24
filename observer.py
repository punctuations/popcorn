# import the modules
import subprocess
import sys
import time
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler


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

        if not ignored:
            if event.event_type == 'created':
                # Event is created, you can process it now
                print(f"Watchdog received created event - {event.src_path}.")
            elif event.event_type == 'modified':
                # Event is modified, you can process it now
                print(f"Watchdog received modified event - {event.src_path}.")


if __name__ == "__main__":
    # Set format for displaying path
    path = sys.argv[1] if len(sys.argv) > 1 else '.'

    # Initialize logging event handler
    event_handler = Handler()

    # Initialize Observer
    observer = Observer()
    observer.schedule(event_handler, path, recursive=True)

    # Start the observer
    observer.start()
    try:
        while True:
            # Set the thread sleep time
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
