import setuptools
import shutil
import os
from setuptools.command.install import install

class PostInstall(install):
    def run(self):
        install.run(self)
        try:
            os.mkdir('/etc/default/clifi')
        except:
            pass
        shutil.copy('default/clifi.cfg', '/etc/default/clifi')
        shutil.copy('default/streams.json', '/etc/default/clifi')

        

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