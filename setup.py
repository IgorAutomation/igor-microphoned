# -*- coding: utf-8 -*-

import os
from typing import Callable
from setuptools import setup, find_packages


def read_file_content(filename: str) -> str:
    with open(filename) as f:
        return f.read()


def read_file_content_as_array(filename: str, split_on: str = "\n", predicate: Callable[[str], bool] = lambda s: True):
    return [line for line in read_file_content(filename).split(split_on) if predicate(line)]


def get_data_files(dir_name):
    files = os.listdir(dir_name)
    ret = list()
    for file in files:
        complete_file = os.path.join(dir_name, file)
        if os.path.isdir(complete_file):
            ret = ret + get_data_files(complete_file)
        else:
            ret.append(complete_file)
    return ret


setup(
    name='igor-microphoned',
    version='0.1.0',
    description='Microphone Daemon for the Igor home automation platform',
    long_description=read_file_content('README.md'),
    author='Igor Team',
    author_email='briandilley@briandilley.com',
    url='https://github.com/IgorAutomation/igor-microphoned',
    license=read_file_content('LICENSE'),
    entry_points={
        'console_scripts': [
            'igormd = microphoned.daemon:daemon_main',
        ],
    },
    packages=find_packages(include=['microphoned', 'microphoned.*']),
    install_requires=read_file_content_as_array(
        "requirements.txt",
        predicate=lambda s: not s.strip().startswith("#") and s is not ""))
