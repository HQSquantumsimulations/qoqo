# qoqo

[![Documentation Status](https://readthedocs.org/projects/qoqo/badge/?version=latest)](https://qoqo.readthedocs.io/en/latest/?badge=latest)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo)](https://pypi.org/project/qoqo/)
![PyPI - License](https://img.shields.io/pypi/l/qoqo)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo)](https://pypi.org/project/qoqo/)

Quantum Operation Quantum Operation  
Yes we use [reduplication](https://en.wikipedia.org/wiki/Reduplication)

qoqo is a python package to represent quantum circuits by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo provides:

* A circuit class to represent quantum programs
* Single- and Two-Qubit Operations that can be executed (decomposed) on any universal quantum computer
* PRAGMA Operations that only apply to certain hardware, simulators or annotate circuits with additional information
* Classical Register and Measurement operations to use in a quantum program
* Measurement classes for evaluating observable measurements based on projective measurements from quantum hardware or simulator readouts
* A Backend base class defining a standard for interfacing from qoqo to other toolkits, hardware and simulators
* A Device base class defining a standard for device representation
* A method to serialize and deserialize circuits via the to_qonfig/from_qonfig functions. These can be used in conjunction with to_json/from_json functions to create a serialized json file of the circuit.

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

## Examples

For an expanding collection of Examples see the jupyter notebook in examples. The examples also require the qoqo_pyquest and qoqo_mock interfaces.

* [Intro example](https://nbviewer.jupyter.org/github/HQSquantumsimulations/qoqo/blob/main/examples/Intro_to_qoqo.ipynb)
