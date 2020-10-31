# -*- coding: utf-8 -*-

from setuptools import setup, find_packages

readme_text: str
with open('README.md') as f:
    readme_text = f.read()

license_text: str
with open('LICENSE') as f:
    license_text = f.read()

setup(
    name='igor-microphoned',
    version='0.1.0',
    description='Microphone Daemon for the Igor home automation platform',
    long_description=readme_text,
    author='Igor Team',
    author_email='briandilley@briandilley.com',
    url='https://github.com/IgorAutomation/igor-microphoned',
    license=license_text,
    entry_points={
        'console_scripts': [
            'igormd = microphoned.daemon:daemon_main',
        ],
    },
    packages=find_packages(exclude=('tests', 'docs')))
