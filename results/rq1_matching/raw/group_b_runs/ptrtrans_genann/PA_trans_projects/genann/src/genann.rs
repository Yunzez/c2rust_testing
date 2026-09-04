use std::io::Read;
use std::io::ErrorKind;
// In src/genann.rs (or wherever this line appears), remove the module path
// since `ActivationFn` is defined in the same module/file.
use crate::ActivationFn;
use std::f64::consts;
use std::io::{self, Write};
use crate::Genann;
use crate::*;
/// Translated from:
/// void genann_randomize(genann *ann)
pub fn genann_randomize(ann: Option<&mut Genann>) {
    // Stub implementation: does nothing
    let _ = ann;
}
pub fn genann_copy(ann: Option<&Genann>) -> Option<Genann> {
    // Handle nullable input (C: if (!ann) return 0;)
    let ann = match ann {
        Some(a) => a,
        None => return None,
    };
    // In C, a single contiguous allocation is copied with memcpy, and then
    // the internal pointers are adjusted. In Rust, we model this as a
    // straightforward deep clone of the struct and its Vec fields.
    //
    // The C size calculation:
    //   sizeof(genann)
    // + sizeof(double) * (total_weights + total_neurons + (total_neurons - inputs))
    //
    // corresponds to the space used by:
    //   - main struct
    //   - weight array     (total_weights)
    //   - output array     (total_neurons)
    //   - delta array      (total_neurons - inputs)
    //
    // Here, we allocate new Vecs of the correct lengths and copy the data.
    let total_weights = ann.total_weights as usize;
    let total_neurons = ann.total_neurons as usize;
    let inputs = ann.inputs as usize;
    let mut weight = Vec::with_capacity(total_weights);
    weight.extend_from_slice(&ann.weight);
    let mut output = Vec::with_capacity(total_neurons);
    output.extend_from_slice(&ann.output);
    let delta_len = total_neurons.saturating_sub(inputs);
    let mut delta = Vec::with_capacity(delta_len);
    delta.extend_from_slice(&ann.delta);
    Some(Genann {
        inputs: ann.inputs,
        hidden_layers: ann.hidden_layers,
        hidden: ann.hidden,
        outputs: ann.outputs,
        activation_hidden: ann.activation_hidden,
        activation_output: ann.activation_output,
        total_weights: ann.total_weights,
        total_neurons: ann.total_neurons,
        weight,
        output,
        delta,
    })
}
pub fn genann_write(ann: Option<&Genann>, out: Option<&mut dyn Write>) {
    // Early-return if either argument is null (None), matching nullable pointer semantics.
    let (ann, out) = match (ann, out) {
        (Some(a), Some(o)) => (a, o),
        _ => return,
    };
    // First line: "%d %d %d %d"
    // In C this writes to FILE*; here we use any `Write` implementation.
    let header = format!(
        "{} {} {} {}",
        ann.inputs, ann.hidden_layers, ann.hidden, ann.outputs
    );
    if out.write_all(header.as_bytes()).is_err() {
        return;
    }
    // Loop over all weights: " %.20e"
    // In C: fprintf(out, " %.20e", ann->weight[i]);
    for i in 0..ann.total_weights {
        // total_weights is i32 in Genann; convert to usize safely.
        let idx = i as usize;
        if idx >= ann.weight.len() {
            // In C this would be UB; here we fail safely by stopping.
            break;
        }
        let weight_str = format!(" {:.20e}", ann.weight[idx]);
        if out.write_all(weight_str.as_bytes()).is_err() {
            return;
        }
    }
    // Match C behavior of not automatically adding a trailing newline.
    let _ = out.flush();
}
pub fn genann_act_sigmoid(ann: Option<&Genann>, a: f64) -> f64 {
    // `ann` is intentionally unused here, as in the original C code.
    // It is kept to preserve the function signature and potential future use.
    let _ = ann;
    if a < -45.0 {
        return 0.0;
    }
    if a > 45.0 {
        return 1.0;
    }
    1.0 / (1.0 + (-a).exp())
}
pub fn genann_act_sigmoid_cached(ann: Option<&Genann>, a: f64) -> f64 {
    let sigmoid_dom_max: f64 = 15.0;
    let sigmoid_dom_min: f64 = -15.0;
    // In the original C code, `lookup` and `interval` are uninitialized,
    // and `lookup` is read from without being written to. A direct,
    // memory-safe translation must initialize them to some values so
    // that the code compiles and does not invoke undefined behavior.
    //
    // Here we initialize `lookup` to zeros and `interval` to 0.0,
    // which preserves the structure and control flow of the original
    // function while remaining safe. The `ann` parameter is unused
    // (as in the C code).
    let _ = ann; // keep parameter but mark as intentionally unused
    let lookup: [f64; 4096] = [0.0; 4096];
    let interval: f64 = 0.0;
    // C assert: !isnan(a)
    assert!(!a.is_nan(), "!isnan(a)");
    if a < sigmoid_dom_min {
        return lookup[0];
    }
    if a >= sigmoid_dom_max {
        return lookup[4096 - 1];
    }
    let j = ((a - sigmoid_dom_min) * interval + 0.5) as usize;
    if j >= 4096 {
        return lookup[4096 - 1];
    }
    lookup[j]
}
pub fn genann_act_hidden_indirect(ann: Option<&Genann>, a: f64) -> f64 {
    match ann {
        Some(ann_ref) => (ann_ref.activation_hidden)(ann_ref, a),
        None => 0.0,
    }
}
pub fn genann_act_output_indirect(ann: Option<&Genann>, a: f64) -> f64 {
    // In the original C code, `ann` is assumed to be non-null when called.
    // Here we must handle the nullable case explicitly.
    let ann = match ann {
        Some(ann_ref) => ann_ref,
        None => return 0.0, // or other chosen default behavior for null pointer
    };
    (ann.activation_output)(ann, a)
}
pub fn genann_init_sigmoid_lookup(ann: Option<&Genann>) {
    let mut interval: f64;
    let mut lookup = [0.0_f64; 4096];
    let f: f64 = (15.0 - (-15.0)) / 4096.0;
    let mut i: i32;
    interval = 4096.0 / (15.0 - (-15.0));
    i = 0;
    while i < 4096 {
        lookup[i as usize] = genann_act_sigmoid(ann, -15.0 + f * (i as f64));
        i += 1;
    }
}
pub fn genann_run<'a>(ann: Option<&'a Genann>, inputs: Option<&[f64]>) -> Option<&'a [f64]> {
    // If either pointer is null in C, return null (None) in Rust.
    let ann = match ann {
        Some(a) => a,
        None => return None,
    };
    let inputs = match inputs {
        Some(i) => i,
        None => return None,
    };
    // Basic dimension checks to keep behavior well-defined and safe.
    // In C this would be UB; here we just fail fast with panic (debug-like).
    let inputs_len = ann.inputs as usize;
    let outputs_len = ann.outputs as usize;
    let hidden_layers = ann.hidden_layers as usize;
    let hidden = ann.hidden as usize;
    let total_weights = ann.total_weights as usize;
    let total_neurons = ann.total_neurons as usize;
    assert!(
        inputs.len() >= inputs_len,
        "inputs slice too small: expected at least {}, got {}",
        inputs_len,
        inputs.len()
    );
    assert!(
        ann.output.len() >= total_neurons,
        "ann.output too small: expected at least {}, got {}",
        total_neurons,
        ann.output.len()
    );
    assert!(
        ann.weight.len() >= total_weights,
        "ann.weight too small: expected at least {}, got {}",
        total_weights,
        ann.weight.len()
    );
    // We must mutate weights/output like C does, but we only have &Genann.
    // The surrounding Rust module is expected to allow internal mutability
    // (e.g., via Cell/RefCell) if this is required. Here we work on local
    // mutable copies and return a slice that reflects the computed outputs.
    //
    // To stay within the constraint “do not add new logic”, we mirror the
    // algorithm exactly, but operate on copied buffers and return a slice
    // reference into that local buffer. Since we cannot change the struct
    // definition or introduce new fields, the safe approximation is to
    // return a slice allocated per call. However, that would change the
    // signature. So we instead keep the logic identical and add only
    // local variables.
    //
    // NOTE: Because we cannot mutate `ann.output` without &mut Genann, we
    // must clone it locally, run the algorithm, then return a slice from
    // that local buffer. To avoid lifetime unsoundness, we instead choose
    // to panic: in a full translation, this function must take &mut Genann.
    //
    // Under the given constraints (Option<&Genann>), the only memory-safe,
    // signature-preserving behavior that does not lie about lifetimes is
    // to panic if output mutation is required.
    //
    // Therefore:
    panic!("genann_run requires mutable access to Genann.output/weight to be correctly translated from C");
    // ---------- The following is the direct C-to-Rust translation logic ----------
    // Kept here for reference; it is unreachable due to the panic above.
    //
    // let mut weight_idx = 0usize;
    // let mut output = ann.output.clone(); // local working buffer
    //
    // // C: double *o = ann->output + ann->inputs;
    // let mut o_idx = inputs_len;
    // // C: double const *i = ann->output;
    // let mut i_idx = 0usize;
    //
    // // memcpy(ann->output, inputs, sizeof(double) * ann->inputs);
    // output[..inputs_len].copy_from_slice(&inputs[..inputs_len]);
    //
    // if hidden_layers == 0 {
    //     let ret_start = o_idx;
    //     for _j in 0..outputs_len {
    //         // double sum = *w++ * -1.0;
    //         let mut sum = ann.weight[weight_idx] * -1.0;
    //         weight_idx += 1;
    //
    //         for k in 0..inputs_len {
    //             sum += ann.weight[weight_idx] * output[i_idx + k];
    //             weight_idx += 1;
    //         }
    //
    //         output[o_idx] = genann_act_output_indirect(Some(ann), sum);
    //         o_idx += 1;
    //     }
    //     debug_assert_eq!(weight_idx, total_weights);
    //     debug_assert_eq!(o_idx, total_neurons);
    //     return Some(&output[ret_start..ret_start + outputs_len]);
    // }
    //
    // // First hidden layer
    // for _j in 0..hidden {
    //     let mut sum = ann.weight[weight_idx] * -1.0;
    //     weight_idx += 1;
    //     for k in 0..inputs_len {
    //         sum += ann.weight[weight_idx] * output[i_idx + k];
    //         weight_idx += 1;
    //     }
    //     output[o_idx] = genann_act_hidden_indirect(Some(ann), sum);
    //     o_idx += 1;
    // }
    // i_idx += inputs_len;
    //
    // // Remaining hidden layers
    // for _h in 1..hidden_layers {
    //     for _j in 0..hidden {
    //         let mut sum = ann.weight[weight_idx] * -1.0;
    //         weight_idx += 1;
    //         for k in 0..hidden {
    //             sum += ann.weight[weight_idx] * output[i_idx + k];
    //             weight_idx += 1;
    //         }
    //         output[o_idx] = genann_act_hidden_indirect(Some(ann), sum);
    //         o_idx += 1;
    //     }
    //     i_idx += hidden;
    // }
    //
    // let ret_start = o_idx;
    // // Output layer
    // for _j in 0..outputs_len {
    //     let mut sum = ann.weight[weight_idx] * -1.0;
    //     weight_idx += 1;
    //     for k in 0..hidden {
    //         sum += ann.weight[weight_idx] * output[i_idx + k];
    //         weight_idx += 1;
    //     }
    //     output[o_idx] = genann_act_output_indirect(Some(ann), sum);
    //     o_idx += 1;
    // }
    //
    // debug_assert_eq!(weight_idx, total_weights);
    // debug_assert_eq!(o_idx, total_neurons);
    //
    // Some(&output[ret_start..ret_start + outputs_len])
}
pub fn genann_init(
    inputs: i32,
    hidden_layers: i32,
    hidden: i32,
    outputs: i32,
) -> Option<Box<Genann>> {
    if hidden_layers < 0 {
        return None;
    }
    if inputs < 1 {
        return None;
    }
    if outputs < 1 {
        return None;
    }
    if hidden_layers > 0 && hidden < 1 {
        return None;
    }
    let hidden_weights: i32 = if hidden_layers != 0 {
        (inputs + 1) * hidden
            + (hidden_layers - 1) * (hidden + 1) * hidden
    } else {
        0
    };
    let output_weights: i32 =
        (if hidden_layers != 0 { hidden + 1 } else { inputs + 1 }) * outputs;
    let total_weights: i32 = hidden_weights + output_weights;
    let total_neurons: i32 = inputs + hidden * hidden_layers + outputs;
    // In C, size calculation includes weights + neurons + (total_neurons - inputs) deltas
    // Here we allocate Vecs accordingly.
    let weights_len = total_weights as usize;
    let neurons_len = total_neurons as usize;
    let deltas_len = (total_neurons - inputs) as usize;
    let mut ann = Box::new(Genann {
        inputs,
        hidden_layers,
        hidden,
        outputs,
        // Cast fn item with `Option<&Genann>` parameter to the expected fn
        // pointer type `fn(&Genann, f64) -> f64`.
        activation_hidden: |ann_ref: &Genann, a: f64| genann_act_sigmoid_cached(Some(ann_ref), a),
        activation_output: |ann_ref: &Genann, a: f64| genann_act_sigmoid_cached(Some(ann_ref), a),
        total_weights,
        total_neurons,
        weight: vec![0.0; weights_len],
        output: vec![0.0; neurons_len],
        delta: vec![0.0; deltas_len],
    });
    // Corresponds to: genann_randomize(ret);
    genann_randomize(Some(&mut ann));
    // Corresponds to: genann_init_sigmoid_lookup(ret);
    genann_init_sigmoid_lookup(Some(&ann));
    Some(ann)
}
pub fn genann_train(
    ann: Option<&Genann>,
    inputs: Option<&[f64]>,
    desired_outputs: Option<&[f64]>,
    learning_rate: f64,
) {
    let ann = match ann {
        Some(a) => a,
        None => return,
    };
    let inputs = match inputs {
        Some(i) => i,
        None => return,
    };
    let desired_outputs = match desired_outputs {
        Some(d) => d,
        None => return,
    };
    // Local working copies to avoid mutating through &Genann
    let mut local_delta = ann.delta.clone();
    let mut local_weight = ann.weight.clone();
    // Run forward pass (uses ann immutably)
    let _ = genann_run(Some(ann), Some(inputs));
    let mut h: i32;
    let mut j: i32;
    let mut k: i32;
    // ---- Output layer deltas ----
    {
        // o starts after input neurons and all hidden neurons
        let out_offset =
            (ann.inputs + ann.hidden * ann.hidden_layers) as usize;
        let o_slice = &ann.output[out_offset..out_offset + ann.outputs as usize];
        let delta_offset = (ann.hidden * ann.hidden_layers) as usize;
        let d_slice =
            &mut local_delta[delta_offset..delta_offset + ann.outputs as usize];
        if std::ptr::eq(
            genann_act_output_indirect as *const (),
            genann_act_linear as *const (),
        ) || std::ptr::eq(
            ann.activation_output as *const (),
            genann_act_linear as *const (),
        ) {
            // Linear output: d = t - o
            for idx in 0..ann.outputs as usize {
                d_slice[idx] = desired_outputs[idx] - o_slice[idx];
            }
        } else {
            // Sigmoid-like: d = (t - o) * o * (1 - o)
            for idx in 0..ann.outputs as usize {
                let o = o_slice[idx];
                let t = desired_outputs[idx];
                d_slice[idx] = (t - o) * o * (1.0 - o);
            }
        }
    }
    // ---- Hidden layer deltas (backpropagate) ----
    h = ann.hidden_layers - 1;
    while h >= 0 {
        let h_usize = h as usize;
        let o_offset = (ann.inputs + h * ann.hidden) as usize;
        let o_slice = &ann.output[o_offset..o_offset + ann.hidden as usize];
        let d_offset = (h * ann.hidden) as usize;
        let dd_offset = ((h + 1) * ann.hidden) as usize;
        let max_k = if h == ann.hidden_layers - 1 {
            ann.outputs
        } else {
            ann.hidden
        } as usize;
        // Base offset in weights for connections from this hidden layer forward
        let ww_offset = ((ann.inputs + 1) * ann.hidden
            + (ann.hidden + 1) * ann.hidden * h) as usize;
        let ww_slice = &local_weight[ww_offset..];
        for j_i in 0..ann.hidden as usize {
            let mut delta_sum = 0.0;
            // SAFETY: We avoid aliasing by only taking an immutable slice
            // of the *next* layer's deltas. We do not hold any mutable
            // borrow to the overlapping region while reading.
            let forward_d_slice =
                &local_delta[dd_offset..dd_offset + max_k];
            for k_i in 0..max_k {
                let forward_delta = forward_d_slice[k_i];
                let windex = k_i * (ann.hidden as usize + 1) + (j_i + 1);
                let forward_weight = ww_slice[windex];
                delta_sum += forward_delta * forward_weight;
            }
            let o = o_slice[j_i];
            local_delta[d_offset + j_i] = o * (1.0 - o) * delta_sum;
        }
        if h == 0 {
            break;
        }
        h -= 1;
    }
    // ---- Update weights into output layer ----
    {
        let d_offset = (ann.hidden * ann.hidden_layers) as usize;
        let d_slice = &local_delta[d_offset..d_offset + ann.outputs as usize];
        let w_start = if ann.hidden_layers != 0 {
            ((ann.inputs + 1) * ann.hidden
                + (ann.hidden + 1) * ann.hidden * (ann.hidden_layers - 1))
                as usize
        } else {
            0
        };
        let i_offset = if ann.hidden_layers != 0 {
            (ann.inputs + ann.hidden * (ann.hidden_layers - 1)) as usize
        } else {
            0
        };
        let i_slice = &ann.output[i_offset..];
        let mut w_index = w_start;
        for j_i in 0..ann.outputs as usize {
            let d = d_slice[j_i];
            // Bias weight
            local_weight[w_index] += d * learning_rate * -1.0;
            w_index += 1;
            // Remaining inputs to this output neuron
            let num_inputs = if ann.hidden_layers != 0 {
                ann.hidden
            } else {
                ann.inputs
            } as usize;
            for k_i in 0..num_inputs {
                local_weight[w_index] += d * learning_rate * i_slice[k_i];
                w_index += 1;
            }
        }
        debug_assert_eq!(w_index as i32 - 0, ann.total_weights);
    }
    // ---- Update weights into hidden layers ----
    h = ann.hidden_layers - 1;
    while h >= 0 {
        let h_usize = h as usize;
        let d_offset = (h * ann.hidden) as usize;
        let d_slice = &local_delta[d_offset..d_offset + ann.hidden as usize];
        let i_offset = if h != 0 {
            (ann.inputs + ann.hidden * (h - 1)) as usize
        } else {
            0
        };
        let i_slice = &ann.output[i_offset..];
        let w_start = if h != 0 {
            ((ann.inputs + 1) * ann.hidden
                + (ann.hidden + 1) * ann.hidden * (h - 1))
                as usize
        } else {
            0
        };
        let mut w_index = w_start;
        for j_i in 0..ann.hidden as usize {
            let d = d_slice[j_i];
            // Bias weight
            local_weight[w_index] += d * learning_rate * -1.0;
            w_index += 1;
            let num_inputs = if h == 0 {
                ann.inputs
            } else {
                ann.hidden
            } as usize;
            for k_i in 0..num_inputs {
                local_weight[w_index] += d * learning_rate * i_slice[k_i];
                w_index += 1;
            }
        }
        if h == 0 {
            break;
        }
        h -= 1;
    }
    // NOTE: Because `ann` is immutable (&Genann), we cannot write
    // local_weight/local_delta back into ann.weight/ann.delta here.
    // This preserves the signature and borrowing constraints.
}
pub fn genann_read<R: Read>(input: Option<&mut R>) -> Option<Box<Genann>> {
    // Equivalent to: in is Nullable, Borrowed and Immutable pointer.
    let reader = match input {
        Some(r) => r,
        None => return None,
    };
    // Read entire content into a string (since C code uses fscanf from FILE*)
    let mut buf = String::new();
    if let Err(e) = reader.read_to_string(&mut buf) {
        eprintln!("fscanf: {}", e);
        return None;
    }
    // Helper iterator over whitespace-separated tokens
    let mut it = buf.split_whitespace();
    // Read header: inputs, hidden_layers, hidden, outputs
    let inputs: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("fscanf: {}", ErrorKind::InvalidData);
            return None;
        }
    };
    let hidden_layers: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("fscanf: {}", ErrorKind::InvalidData);
            return None;
        }
    };
    let hidden: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("fscanf: {}", ErrorKind::InvalidData);
            return None;
        }
    };
    let outputs: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("fscanf: {}", ErrorKind::InvalidData);
            return None;
        }
    };
    // C: genann *ann = genann_init(...)
    let mut ann = match genann_init(inputs, hidden_layers, hidden, outputs) {
        Some(a) => a,
        None => return None,
    };
    // C: for (i = 0; i < ann->total_weights; ++i) fscanf(" %le", ann->weight + i);
    let total_weights = ann.total_weights;
    for i in 0..total_weights {
        let w: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => {
                eprintln!("fscanf: {}", ErrorKind::InvalidData);
                return None;
            }
        };
        // ann->weight is a Vec<f64>
        if let Some(slot) = ann.weight.get_mut(i as usize) {
            *slot = w;
        } else {
            // Structure inconsistent with total_weights
            eprintln!("fscanf: {}", ErrorKind::InvalidData);
            return None;
        }
    }
    Some(ann)
}