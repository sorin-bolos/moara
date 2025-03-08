import init, {simulate, get_statevector, get_probabilities} from "./pkg/moara_js.js";
function sim() 
    { 
        console.info("sim");
        init().then(() => {
            let circ = document.getElementById('circuit').value;
            console.log("sim + " + circ);
            let r = simulate(circ, 1024, 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

function get_sv() 
    { 
        init().then(() => {
            let circ = document.getElementById('circuit').value;
            console.log("get_sv + " + circ);
            let r = get_statevector(circ, "bigendian", 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

function get_probs() 
    { 
        init().then(() => {
            let circ = document.getElementById('circuit').value;
            console.log("get_probs + " + circ);
            let r = get_probabilities(circ, "bigendian", 2)
            document.getElementById('demo').innerHTML = r;
        });
    };

window.sim = sim;
window.get_sv = get_sv;
window.get_probs = get_probs;

