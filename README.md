# moara
Moara is a high performance Quantum Computer Simulator. It simulates a system with no noise.
It comes in three flawors: the stand alone, executable, called simply **moara**; the version that can be used in Qiskit **moara-for-qiskit** and the version dimmed **pymoara** that can be used with PyQuil and Cirq.

Note: The release versions are only built for use with Windows.

## moara-for-qiskit
Installation
```
pip install moara_for_qiskit
```

Usage
```
from qiskit import QuantumCircuit, execute
circuit = QuantumCircuit(2)
circuit.h(0)
circuit.cx(0,1)
circuit.measure_all()

from moara_for_qiskit import MoaraBackend
simulator = MoaraBackend()
result = execute(circuit, simulator, shots=1024)
print(result)
```
By default *MoaraBackend* returns bitstrings in big-endian form (the reverse of what native Qiskit backends return). The left-most bit in the bitstring corresponds to the measured value of the firs qubit in the circuit (qubit[0]). This can be changed by a parameter in the constructor: `MoaraBackend(little_endian=True)`.
Qiskit needs to be installed in the enviromnent in order to use `moara-for-qiskit`
**Note:** *MoaraBackend* threats all qubits as being measured at the end of the circuit (even the ones that don't have a measurement gate). The bitstrings are constructed in the order of qubits and do not follow the order of classical bits in the measurement gate formats. 

## pymoara

Pymoara has a stand alone simulator for use in python. This simulator supports circuits in the quil format and in the format used by cirq.

Installation
```
pip install pymoara
```

### Usage with pyquil
```
from pyquil import Program, get_qc
from pyquil.gates import *

program = Program()
ro = program.declare('ro', 'BIT', 2)
program += H(0)
program += CNOT(0,1)
program += MEASURE(0, ro[0])
program += MEASURE(1, ro[1])

from pymoara import MoaraSimulator
simulator = MoaraSimulator()
result = simulator.run(program)
print(result)
```

Supported quil gates and modifiers
```
# Gates
I, X, Y, Z, H, S, RX(%theta), RY(%theta), RZ(%theta), PHASE(%theta), CNOT, CZ

# Modifiers
CONTROLLED #once
DAGGER
```

### Usage with cirq
```
import cirq
from cirq.ops import *

qb = cirq.LineQubit.range(3)
circ = cirq.Circuit()
circ.append(H(qb[0]))
circ.append(CNOT(qb[0], qb[1]))
circ.append(cirq.measure(qb[0]))
circ.append(cirq.measure(qb[1]))

from pymoara import MoaraSimulator
simulator = MoaraSimulator()
result = simulator.run(circ)
print(result)
```

Supported cirq gates
```
MeasurementGate, IdentityGate, X, Y, Z, rx, ry, rz, XPowGate, YPowGat, ZPowGate, S, T, H, CNOT, CZPowGate
```

## Result format
The result is returned as an array of 2^n values. The value at index i corresponds to the number of samples collected for the bitsring i.
**Note:** *MoaraSimulator* threats all qubits as being measured at the end of the circuit (even the ones that don't have a measurement gate). The bitstrings are constructed in the order of qubits and do not follow the order of classical bits in the measurement gate formats present in pyquil. 

## moara.exe
To use moara for uranium the circuit to be run needs to be saved in json for according to the below model;

*circuit.json*
```
{
  "steps": [
    {
      "index": 0,
      "gates": [
        {
          "name": "hadamard",
          "target": 0
        },
        {
          "name": "pauli-x",
          "target": 1
        }
      ]
    },
    {
      "index": 1,
      "gates": [
        {
          "name": "ctrl-pauli-x",
          "target": 1,
          "control": 0
        }
      ]
    },
  ]
}
```
Then you can run the circuit using the following command:
```
moara_for_uranium.exe circuit_file [no_of_shots] [qubit_count]

ex1. moara_for_uranium.exe circuit.json
ex2. moara_for_uranium.exe circuit.json 1024 3
```
If `no_of_shots` is not specified the defauld of `1024` is used.
If `qubit_count` is not specified the number of qubits is infered from the circuit json.

### Circuit json structure
The circuit json is composed of a main circuit object that has one element called `steps`. This element is a list of step objects.
Each step object has an index of integer type and a list of `gates`.
Each gate object has a `name (string)` and a `target (integer)`. Some gates may have other parameters like `control (integer)`, `phi (numeric)`, `theta (numeric)` and `lambda (numeric)`

#### Available gates:
```
pauli-x,
pauli-y,
pauli-z,
hadamard,
t,
t-dagger,
s,
s-dagger,
sqrt-not,
u, #(phi, theta, labmda)
u-phi-theta,
r-phi,
rx-phi,
ry-phi,
rz-phi,
ctrl-pauli-x,
ctrl-pauli-y,
ctrl-pauli-z,
ctrl-hadamard,
ctrl-t,
ctrl-t-dagger,
ctrl-s,
ctrl-s-dagger,
ctrl-sqrt-not,
ctrl-u,
ctrl-u-phi-theta,
ctrl-r-phi,
ctrl-rx-phi,
ctrl-ry-phi,
ctrl-rz-phi
```
