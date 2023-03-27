import pytest
import sys
import qoqo


def test_circuit_init():
    circuit = qoqo.Circuit()
    circuit += qoqo.operations.PauliX(0)


if __name__ == "__main__":
    """The main if called as script."""
    pytest.main(sys.argv)
