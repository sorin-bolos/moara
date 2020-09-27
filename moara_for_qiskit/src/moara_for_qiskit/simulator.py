from qiskit.providers.models import QasmBackendConfiguration
from qiskit.providers import BaseBackend
import moara_interop as moara
import json

class MoaraSimulator(BaseBackend):
    
    NAME = 'moara_for_qiskit'
    VERSION = '0.1'
    MAX_QUBITS_COUNT = 20
    MAX_SHOTS = int(1e6)

    CONFIGURATION = { 'backend_name': NAME,
        'backend_version': VERSION,
        'n_qubits': MAX_QUBITS_COUNT,
        'simulator': True,
        'local': True,
        'conditional': True,
        'memory': False,
        'open_pulse': False,
        'max_shots': MAX_SHOTS,
        'description': 'A simulator',
        'coupling_map': None,
        'basis_gates': ['cx','id', 'x', 'y', 'z', 'h','swap'],
        'gates': []
}

    def __init__(self):
       super().__init__(QasmBackendConfiguration.from_dict(self.CONFIGURATION), None)

    def run(self, qobj, backend_options=None, noise_model=None, validate=False):
        if not qobj or not qobj.experiments:
            return {}
        experiment = qobj.experiments[0]

        circuit = { "steps":[] }
        index = 0
        for instruction in experiment.instructions:
            if not instruction or len(instruction.qubits) == 0:
                continue

            gate = None
            if instruction.name == 'id':
                gate = { 'name': 'identity', 'target':instruction.qubits[0] }
            elif instruction.name == 'x':
                gate = { 'name': 'pauli-x', 'target':instruction.qubits[0] }
            elif instruction.name == 'y':
                gate = { 'name': 'pauli-y', 'target':instruction.qubits[0] }
            elif instruction.name == 'z':
                gate = { 'name': 'pauli-z', 'target':instruction.qubits[0] }
            elif instruction.name == 'h':
                gate = { 'name': 'hadamard', 'target':instruction.qubits[0] }
            elif instruction.name == 'u3':
                gate = { 'name': 'u3', 'target':instruction.qubits[0] }
            elif instruction.name == 'cx':
                if len(instruction.qubits) != 2:
                    continue
                gate = { 'name': 'ctrl-pauli-x', 'control':instruction.qubits[0], 'target':instruction.qubits[1] }
            elif instruction.name == 'swap':
                if len(instruction.qubits) != 2:
                    continue
                gate = { 'name': 'swap', 'target':instruction.qubits[0], 'target2':instruction.qubits[1] }

            if gate:
                step = { 'index':index, 'gates':[gate] }
                circuit['steps'].append(step)
                index += 1

        serializedCircuit = json.dumps(circuit)

        return moara.simulate(serializedCircuit, qobj.config.shots, experiment.config.n_qubits)