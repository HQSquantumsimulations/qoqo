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
"""Testing example notebooks"""

import pytest
import sys
import nbformat
from nbconvert.preprocessors import ExecutePreprocessor
import os
from pathlib import Path


def test_intro_to_qoqo():
    """Test cheated basis rotation measurement input using the init function"""
    path = os.path.dirname(Path(os.path.abspath(__file__)).parents[1])
    with open(os.path.join(path, 'examples/Intro_to_qoqo.ipynb'), 'r') as file:
        notebook = nbformat.read(file, as_version=4)
    executor = ExecutePreprocessor(timeout=120)
    executor.preprocess(notebook)


if __name__ == '__main__':
    pytest.main(sys.argv)
