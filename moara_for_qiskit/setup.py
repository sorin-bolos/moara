from os.path import basename, splitext
from glob import glob

from setuptools import setup, find_packages
from src.moara_for_qiskit import __version__

setup(
    name="moara_for_qiskit",
    version=__version__,
    author="Sorin Bolos",
    author_email="sorinalbolos@yahoo.com",
    description="A simulator for use with Qiskit",
    url="https://github.com/sorin-bolos/moara.git",
    packages=find_packages('src'),
    package_dir={'': 'src'},
    py_modules=[splitext(basename(path))[0] for path in glob('src/*.py')],
    install_requires=[
        'qiskit',
        'matplotlib'
    ],
    keywords='quantum programming'
)