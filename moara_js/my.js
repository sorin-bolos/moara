import init, {simulate, get_statevector, get_probabilities} from "./pkg/moara_js.js";
function sim() 
    { 
        init().then(() => {
            let circ = '{"steps": [{"index": 2,"gates": []},{"index": 1,"gates": [{"name": "ctrl-pauli-x","target": 1,"control": 0}]},{"index": 0,"gates": [{"name": "hadamard","target": 0}]}]}';
            let r = simulate(circ, 1024, 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

function get_sv() 
    { 
        init().then(() => {
            let circ = '{"steps": [{"index": 2,"gates": []},{"index": 1,"gates": [{"name": "ctrl-pauli-x","target": 1,"control": 0}]},{"index": 0,"gates": [{"name": "hadamard","target": 0}]}]}';
            let r = get_statevector(circ, 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

function get_probs() 
    { 
        init().then(() => {
            let circ = '{"steps": [{"index": 2,"gates": []},{"index": 1,"gates": [{"name": "ctrl-pauli-x","target": 1,"control": 0}]},{"index": 0,"gates": [{"name": "hadamard","target": 0}]}]}';
            let r = get_probabilities(circ, 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

window.sim = sim;
window.get_sv = get_sv;
window.get_probs = get_probs;

