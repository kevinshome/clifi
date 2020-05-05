import setuptools

setuptools.setup(
    name="clifi",
    version="520.1",
    author="kevinshome",
    author_email="",
    description="",
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
)