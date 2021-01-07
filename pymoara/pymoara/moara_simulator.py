import json
from .pymoara import simulate
from .cirq_parser import CirqParser
from .pyquil_parser import PyquilParser

class MoaraSimulator():

    def run(self, circuit, shots=1024, littleEndian=False):
        if not circuit:
            return {}
        
        moara_circuit = None
        if hasattr(circuit, '_moments'):
            moara_circuit, qubits = CirqParser().parse(circuit)
        elif hasattr(circuit, '_instructions'):
            moara_circuit, qubits = PyquilParser().parse(circuit)
        elif hasattr(circuit, '_data'):
            moara_circuit, qubits = self._parse_qiskit(circuit._data)

        if moara_circuit == None:
            raise Exception("Could not detect circuit source")

        serializedCircuit = json.dumps(moara_circuit)
        
        return simulate(serializedCircuit, shots, qubits)
