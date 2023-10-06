# Styled off of https://github.com/orbcode/orbtrace/blob/afbe0b95a1294d4e0fe6a193a44000554c0773cd/deps/__init__.py
import sys
import os

# Set up the import paths for the LiteX packages
def listdirs(path: str):
    return [d for d in os.listdir(path) if os.path.isdir(os.path.join(path, d))]

rootdir = os.path.dirname(__file__)
directories = [rootdir + "/" + directory_name for directory_name in listdirs(rootdir)]

for path in directories:
    # Some things (LiteX) don't like to play nice with PYTHONPATH, so we override them by inserting at the beginning
    sys.path.insert(0, path)

# Set PYTHONPATH so child processes inherit these types
os.environ["PYTHONPATH"] = ":".join(directories)