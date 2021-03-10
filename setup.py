# Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Install qoqo"""

from setuptools import find_packages, setup
import os
path = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(path, 'README.md')) as file:
    readme = file.read()

License = 'Apache-2.0'


# obtain current version
__version__ = None
with open(os.path.join(path, 'qoqo/__version__.py')) as f:
    lines = f.readlines()
__version__ = lines[-1].strip().split("'")[1].strip()

install_requires = [
    'hqsbase>=0.7.5',
    'numpy',
    'scipy',
    'networkx',
    'pandas'
]

authors = 'HQS Quantum Simulations'

setup(name='qoqo',
      description='Quantum Computing Base Package',
      version=__version__,
      long_description=readme,
      packages=find_packages(exclude=('docs')),
      author=authors,
      author_email='info@quantumsimulations.de',
      url='https://quantumsimulations.de',
      license=License,
      python_requires='>=3.7',
      install_requires=install_requires,
      )
