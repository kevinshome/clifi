import setuptools
import shutil
import os
from setuptools.command.install import install
import sys

class PostInstall(install):
    def run(self):
        install.run(self)
        if sys.platform != 'win32':
            if not os.path.exists('/etc/default/clifi'):
                os.mkdir('/etc/default/clifi')
            shutil.copy('default/clifi.cfg', '/etc/default/clifi')
            shutil.copy('default/streams.json', '/etc/default/clifi')
        else:
            from pathlib import Path
            home = str(Path.home())
            if not os.path.exists('{}\.clifi'.format(home)):
                os.mkdir('{}\.clifi'.format(home))
            defaults = '{}\.clifi\defaults'.format(home)
            if not os.path.exists(defaults):
                    os.mkdir(defaults)
            shutil.copy('default/clifi.cfg', defaults)
            shutil.copy('default/streams.json', defaults)
        

setuptools.setup(
    name="clifi",
    version="520.3",
    author="kevinshome",
    description="play your favorite streams straight from the command line",
    url="https://github.com/kevinshome/clifi",
    packages=setuptools.find_packages(
        where="src",
    ),
    scripts=['bin/clifi'],
    package_dir={"": "src"},
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.2.6',
    cmdclass={
        'install': PostInstall,
    },
)