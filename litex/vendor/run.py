import __init__

import sys
import subprocess

if len(sys.argv) < 2:
  print("No arguments provided. Doing nothing")
  sys.exit(1)

subprocess.call(sys.argv[1], shell=True)