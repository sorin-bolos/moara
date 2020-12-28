# moara
Moara is a high performance Quantum Computer Simulator.
It comes in two flawors: the stand alone, executable, version dimmed **moara-for-uranium** and the one that can be used in Qiskit **moara-for-qiskit**.

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

from moara_for_qiskit import MoaraSimulator
simulator = MoaraSimulator()
result = execute(circuit, simulator, shots=1024)
print(result)
```
**Note:** **mora_for_qiskit** simulator returns the bitstrings in Big Endian form.
Meaning that the qubit 0 (the highest in the circuit design) is represented in the left-most classical bit in the returned bitstring.
This is the reverse of the way Qiskit's usual backends return the bitstrings.

## moara-for-uranium
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
