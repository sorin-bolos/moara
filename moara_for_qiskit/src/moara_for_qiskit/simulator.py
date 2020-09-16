from qiskit.providers.models import QasmBackendConfiguration
from qiskit.providers import BaseBackend

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
        'basis_gates': ['u3', 'cx','id', 'x', 'y', 'z', 'h','swap'],
        'gates': []
}

    def __init__(self):
       super().__init__(QasmBackendConfiguration.from_dict(self.CONFIGURATION), None)

    def run(self, qobj, backend_options=None, noise_model=None, validate=False):
        return qobj #{'01':534, '10':456}