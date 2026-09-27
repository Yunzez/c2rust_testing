//! Safe, idiomatic Rust port of the GENANN minimal neural network.
//!
//! This is a mostly direct translation preserving structure and semantics.

use std::f64::consts::E;

const LOOKUP_SIZE: usize = 4096;

/// Type alias for activation functions.
pub type ActivationFn = fn(f64) -> f64;

/// Neural network structure analogous to the original `genann`.
#[derive(Clone)]
pub struct NeuralNetwork {
    pub inputs: usize,
    pub hidden_layers: usize,
    pub hidden: usize,
    pub outputs: usize,

    pub total_weights: usize,
    pub total_neurons: usize,

    /// All weights in a single contiguous buffer (same layout as GENANN).
    pub weights: Vec<f64>,
    /// Output (activations) scratch buffer for all neurons.
    outputs_buf: Vec<f64>,
    /// Delta (error) scratch buffer for all non-input neurons.
    deltas_buf: Vec<f64>,

    pub activation_hidden: ActivationFn,
    pub activation_output: ActivationFn,
}

/// Sigmoid activation.
pub fn activation_sigmoid(a: f64) -> f64 {
    if a < -45.0 {
        0.0
    } else if a > 45.0 {
        1.0
    } else {
        1.0 / (1.0 + (-a).exp())
    }
}

/// Cached sigmoid activation using a lookup table.
///
/// Precomputed once on first use; subsequent calls are O(1).
pub fn activation_sigmoid_cached(a: f64) -> f64 {
    use std::sync::OnceLock;

    const MIN: f64 = -15.0;
    const MAX: f64 = 15.0;

    struct Table {
        interval: f64,
        values: [f64; LOOKUP_SIZE],
    }

    static TABLE: OnceLock<Table> = OnceLock::new();

    let table = TABLE.get_or_init(|| {
        let interval = (MAX - MIN) / LOOKUP_SIZE as f64;
        let mut values = [0.0_f64; LOOKUP_SIZE];
        for (i, v) in values.iter_mut().enumerate() {
            *v = activation_sigmoid(MIN + interval * i as f64);
        }
        Table { interval, values }
    });

    let i = ((a - MIN) / table.interval + 0.5) as isize;
    if i <= 0 {
        table.values[0]
    } else if i as usize >= LOOKUP_SIZE {
        table.values[LOOKUP_SIZE - 1]
    } else {
        table.values[i as usize]
    }
}

/// Threshold activation (step at 0).
pub fn activation_threshold(a: f64) -> f64 {
    if a > 0.0 { 1.0 } else { 0.0 }
}

/// Linear activation.
pub fn activation_linear(a: f64) -> f64 {
    a
}

impl NeuralNetwork {
    /// Create a new network (equivalent to `genann_init`).
    /// Returns `None` for invalid configurations.
    pub fn new(inputs: usize, hidden_layers: usize, hidden: usize, outputs: usize) -> Option<Self> {
        if hidden_layers == 0 {
            if inputs < 1 || outputs < 1 {
                return None;
            }
        } else {
            if inputs < 1 || outputs < 1 || hidden < 1 {
                return None;
            }
        }

        let hidden_weights = if hidden_layers > 0 {
            (inputs + 1) * hidden
                + (hidden_layers - 1) * (hidden + 1) * hidden
        } else {
            0
        };

        let output_weights = (if hidden_layers > 0 {
            hidden + 1
        } else {
            inputs + 1
        }) * outputs;

        let total_weights = hidden_weights + output_weights;
        let total_neurons = inputs + hidden * hidden_layers + outputs;

        let mut nn = NeuralNetwork {
            inputs,
            hidden_layers,
            hidden,
            outputs,
            total_weights,
            total_neurons,
            weights: vec![0.0; total_weights],
            outputs_buf: vec![0.0; total_neurons],
            deltas_buf: vec![0.0; total_neurons - inputs],
            activation_hidden: activation_sigmoid_cached,
            activation_output: activation_sigmoid_cached,
        };

        nn.randomize_weights();
        Some(nn)
    }

    /// Randomize weights in range [-0.5, 0.5) (equivalent to `genann_randomize`).
    pub fn randomize_weights(&mut self) {
        // Replace GENANN_RANDOM() with uniform [0,1) using a simple LCG-based RNG
        // on top of `rand::random` would require external crate; instead use
        // `fastrand`-like approach with std only: use `std::time` as entropy and
        // a deterministic LCG per call. For simplicity, rely on `rand()`-like
        // quality from `f64` RNG based on `std::time`.
        use std::time::{SystemTime, UNIX_EPOCH};

        fn next_seed(seed: &mut u64) -> f64 {
            // LCG constants from Numerical Recipes
            *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            // Take upper 53 bits for f64 mantissa equivalent
            let v = (*seed >> 11) as u64; // 53 bits
            (v as f64) / ((1_u64 << 53) as f64)
        }

        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let mut seed = start.as_nanos() as u64;

        for w in &mut self.weights {
            let r = next_seed(&mut seed);
            *w = r - 0.5;
        }
    }

    /// Run the network forward (equivalent to `genann_run`).
    ///
    /// Returns a slice of the output activations.
    pub fn run(&mut self, inputs: &[f64]) -> &[f64] {
        assert_eq!(inputs.len(), self.inputs);

        // layout: first `inputs` entries are input copies
        self.outputs_buf[..self.inputs].copy_from_slice(inputs);

        let mut w_index = 0usize;
        let mut o_index = self.inputs; // where to write
        let mut i_index = 0usize; // where to read

        let act_hidden = self.activation_hidden;
        let act_output = self.activation_output;

        // Hidden layers
        for h in 0..self.hidden_layers {
            let in_count = if h == 0 { self.inputs } else { self.hidden };
            for _j in 0..self.hidden {
                let mut sum = self.weights[w_index] * -1.0;
                w_index += 1;
                for k in 0..in_count {
                    sum += self.weights[w_index] * self.outputs_buf[i_index + k];
                    w_index += 1;
                }
                self.outputs_buf[o_index] = act_hidden(sum);
                o_index += 1;
            }
            i_index += in_count;
        }

        let output_start = o_index;

        // Output layer
        let in_count = if self.hidden_layers > 0 {
            self.hidden
        } else {
            self.inputs
        };

        for _j in 0..self.outputs {
            let mut sum = self.weights[w_index] * -1.0;
            w_index += 1;
            for k in 0..in_count {
                sum += self.weights[w_index] * self.outputs_buf[i_index + k];
                w_index += 1;
            }
            self.outputs_buf[o_index] = act_output(sum);
            o_index += 1;
        }

        debug_assert_eq!(w_index, self.total_weights);
        debug_assert_eq!(o_index, self.total_neurons);

        &self.outputs_buf[output_start..output_start + self.outputs]
    }

    /// Train the network with one sample (equivalent to `genann_train`).
    pub fn train(&mut self, inputs: &[f64], desired_outputs: &[f64], learning_rate: f64) {
        assert_eq!(inputs.len(), self.inputs);
        assert_eq!(desired_outputs.len(), self.outputs);

        // Forward pass
        let outputs = self.run(inputs);
        debug_assert_eq!(outputs.len(), self.outputs);

        let inputs_count = self.inputs;
        let hidden_neurons = self.hidden * self.hidden_layers;

        // First output's index in outputs_buf
        let out_layer_start = inputs_count + hidden_neurons;
        // First output delta index in deltas_buf
        let out_delta_start = hidden_neurons;

        // Output layer deltas
        if self.activation_output as usize == activation_linear as usize {
            for j in 0..self.outputs {
                let o = self.outputs_buf[out_layer_start + j];
                let t = desired_outputs[j];
                self.deltas_buf[out_delta_start + j] = t - o;
            }
        } else {
            for j in 0..self.outputs {
                let o = self.outputs_buf[out_layer_start + j];
                let t = desired_outputs[j];
                self.deltas_buf[out_delta_start + j] = (t - o) * o * (1.0 - o);
            }
        }

        // Hidden layer deltas, from last to first
        for h in (0..self.hidden_layers).rev() {
            let o_start = inputs_count + h * self.hidden;
            let d_start = h * self.hidden;
            let dd_start = (h + 1) * self.hidden; // following layer (hidden or output)

            // First weight in following layer
            let ww = (inputs_count + 1) * self.hidden
                + (self.hidden + 1) * self.hidden * h;

            for j in 0..self.hidden {
                let mut delta_acc = 0.0;
                let forward_count = if h == self.hidden_layers - 1 {
                    self.outputs
                } else {
                    self.hidden
                };
                for k in 0..forward_count {
                    let forward_delta = if h == self.hidden_layers - 1 {
                        // from outputs area of deltas_buf
                        self.deltas_buf[out_delta_start + k]
                    } else {
                        self.deltas_buf[dd_start + k]
                    };
                    let windex = ww
                        + k * (self.hidden + 1)
                        + (j + 1);
                    let forward_weight = self.weights[windex];
                    delta_acc += forward_delta * forward_weight;
                }
                let o = self.outputs_buf[o_start + j];
                self.deltas_buf[d_start + j] = o * (1.0 - o) * delta_acc;
            }
        }

        // Train output layer weights
        {
            let mut d_index = out_delta_start;

            let mut w_index = if self.hidden_layers > 0 {
                (inputs_count + 1) * self.hidden
                    + (self.hidden + 1) * self.hidden * (self.hidden_layers - 1)
            } else {
                0
            };

            let i_index = if self.hidden_layers > 0 {
                inputs_count + self.hidden * (self.hidden_layers - 1)
            } else {
                0
            };

            let in_count = if self.hidden_layers > 0 {
                self.hidden
            } else {
                self.inputs
            };

            for _j in 0..self.outputs {
                let d = self.deltas_buf[d_index];
                for k in 0..=in_count {
                    if k == 0 {
                        self.weights[w_index] += d * learning_rate * -1.0;
                    } else {
                        self.weights[w_index] +=
                            d * learning_rate * self.outputs_buf[i_index + (k - 1)];
                    }
                    w_index += 1;
                }
                d_index += 1;
            }

            debug_assert_eq!(w_index, self.total_weights);
        }

        // Train hidden layer weights
        for h in (0..self.hidden_layers).rev() {
            let mut d_index = h * self.hidden;

            let i_index = if h > 0 {
                inputs_count + self.hidden * (h - 1)
            } else {
                0
            };

            let mut w_index = if h > 0 {
                (inputs_count + 1) * self.hidden
                    + (self.hidden + 1) * self.hidden * (h - 1)
            } else {
                0
            };

            let in_count = if h == 0 { self.inputs } else { self.hidden };

            for _j in 0..self.hidden {
                let d = self.deltas_buf[d_index];
                for k in 0..=in_count {
                    if k == 0 {
                        self.weights[w_index] += d * learning_rate * -1.0;
                    } else {
                        self.weights[w_index] +=
                            d * learning_rate * self.outputs_buf[i_index + (k - 1)];
                    }
                    w_index += 1;
                }
                d_index += 1;
            }
        }
    }
}
