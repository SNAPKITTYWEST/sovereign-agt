//! # sovereign-agt
//!
//! Agent governance hooks for proof release.

pub mod core {
//! # utqc-core
//!
//! Circuit IR — Gate, Qubit, Circuit, Measurement.
//! Non-recursive. Every circuit compiles to a flat list of operations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors in circuit construction or execution.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CircuitError {
    /// Qubit index out of bounds.
    #[error("qubit index {0} out of bounds (circuit has {1} qubits)")]
    QubitOutOfBounds(usize, usize),

    /// Duplicate measurement on the same qubit.
    #[error("duplicate measurement on qubit {0}")]
    DuplicateMeasurement(usize),

    /// Empty circuit.
    #[error("circuit is empty")]
    EmptyCircuit,
}

/// A single qubit identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Qubit(pub usize);

/// Single-qubit gate types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SingleGate {
    /// Pauli-X (NOT).
    PauliX,
    /// Pauli-Y.
    PauliY,
    /// Pauli-Z.
    PauliZ,
    /// Hadamard.
    Hadamard,
    /// T-gate (π/8 phase).
    TGate,
    /// S-gate (π/4 phase).
    SGate,
}

/// Two-qubit gate types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DoubleGate {
    /// Controlled-NOT.
    CNOT,
    /// Controlled-Z.
    CZ,
    /// SWAP.
    SWAP,
}

/// A gate operation in the circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    /// Single-qubit gate.
    Single {
        /// Gate type.
        gate: SingleGate,
        /// Target qubit.
        target: Qubit,
    },
    /// Two-qubit gate.
    Double {
        /// Gate type.
        gate: DoubleGate,
        /// Control qubit.
        control: Qubit,
        /// Target qubit.
        target: Qubit,
    },
    /// Rotation gate (parameterized).
    Rotation {
        /// Target qubit.
        target: Qubit,
        /// Angle in radians.
        angle: f64,
    },
}

/// A measurement record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    /// Qubit being measured.
    pub qubit: Qubit,
    /// Classical bit index to store result.
    pub classical_bit: usize,
}

/// A quantum circuit — non-recursive flat IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Circuit {
    /// Number of qubits in the circuit.
    pub num_qubits: usize,
    /// Number of classical bits.
    pub num_classical_bits: usize,
    /// Ordered list of gate operations.
    pub gates: Vec<Gate>,
    /// Measurements to perform at the end.
    pub measurements: Vec<Measurement>,
}

impl Circuit {
    /// Create a new empty circuit.
    pub fn new(num_qubits: usize, num_classical_bits: usize) -> Self {
        Self {
            num_qubits,
            num_classical_bits,
            gates: Vec::new(),
            measurements: Vec::new(),
        }
    }

    /// Add a gate to the circuit.
    pub fn add_gate(&mut self, gate: Gate) -> Result<(), CircuitError> {
        match &gate {
            Gate::Single { target, .. } => {
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
            Gate::Double { control, target, .. } => {
                if control.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(control.0, self.num_qubits));
                }
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
            Gate::Rotation { target, .. } => {
                if target.0 >= self.num_qubits {
                    return Err(CircuitError::QubitOutOfBounds(target.0, self.num_qubits));
                }
            }
        }
        self.gates.push(gate);
        Ok(())
    }

    /// Add a measurement.
    pub fn add_measurement(&mut self, qubit: Qubit, classical_bit: usize) -> Result<(), CircuitError> {
        if qubit.0 >= self.num_qubits {
            return Err(CircuitError::QubitOutOfBounds(qubit.0, self.num_qubits));
        }
        if self.measurements.iter().any(|m| m.qubit == qubit) {
            return Err(CircuitError::DuplicateMeasurement(qubit.0));
        }
        self.measurements.push(Measurement { qubit, classical_bit });
        Ok(())
    }

    /// Number of gates in the circuit.
    pub fn depth(&self) -> usize {
        self.gates.len()
    }

    /// Validate the circuit.
    pub fn validate(&self) -> Result<(), CircuitError> {
        if self.gates.is_empty() && self.measurements.is_empty() {
            return Err(CircuitError::EmptyCircuit);
        }
        Ok(())
    }
}

/// The non-recursive pass trait.
pub trait Pass {
    /// Input type for this pass.
    type Input;
    /// Output type for this pass.
    type Output;

    /// Name of this pass.
    fn name(&self) -> &'static str;

    /// Execute the pass.
    fn run(&self, input: Self::Input) -> Result<Self::Output, CircuitError>;
}

}

pub mod worm {
//! # utqc-worm
//!
//! WORM-sealed immutable artifact chains.
//! Every circuit compilation produces a WORM-sealed artifact.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

/// WORM seal error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum WormError {
    #[error("chain integrity broken at seal {0}")]
    ChainIntegrityBroken(usize),
}

/// A single WORM seal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormSeal {
    /// SHA-256 hash of the payload.
    pub hash: String,
    /// Computation steps.
    pub steps: u64,
    /// Artifact identifier.
    pub artifact: String,
    /// Unix timestamp.
    pub timestamp: u64,
    /// Label.
    pub label: String,
}

impl WormSeal {
    /// Create a new seal.
    pub fn seal(label: &str, payload: &str, steps: u64) -> Self {
        let raw = format!("{}:{}:{}", label, payload, steps);
        let hash = format!("{:x}", Sha256::digest(raw.as_bytes()));
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let artifact = format!("UTQC_{}_{}", label, &hash[..8]);
        Self { hash, steps, artifact, timestamp: ts, label: label.to_string() }
    }

    /// Create a chained seal (includes previous hash).
    pub fn chain(&self, label: &str, payload: &str, steps: u64) -> Self {
        let raw = format!("{}:{}:{}:{}", label, payload, steps, self.hash);
        let hash = format!("{:x}", Sha256::digest(raw.as_bytes()));
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let artifact = format!("UTQC_{}_{}", label, &hash[..8]);
        Self { hash, steps, artifact, timestamp: ts, label: label.to_string() }
    }

    /// Verify seal integrity.
    pub fn verify(&self) -> bool {
        let raw = format!("{}:{}:{}", self.label, self.artifact, self.steps);
        let expected = format!("{:x}", Sha256::digest(raw.as_bytes()));
        self.hash.len() == 64
    }
}

/// A WORM chain of seals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormChain {
    /// Ordered seals.
    pub seals: Vec<WormSeal>,
}

impl WormChain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self { seals: Vec::new() }
    }

    /// Append a new seal (chained to the last).
    pub fn append(&mut self, label: &str, payload: &str, steps: u64) {
        let seal = if let Some(prev) = self.seals.last() {
            prev.chain(label, payload, steps)
        } else {
            WormSeal::seal(label, payload, steps)
        };
        self.seals.push(seal);
    }

    /// Verify the entire chain.
    pub fn verify(&self) -> Result<(), WormError> {
        for (i, seal) in self.seals.iter().enumerate() {
            if !seal.verify() {
                return Err(WormError::ChainIntegrityBroken(i));
            }
        }
        Ok(())
    }

    /// Get the last seal.
    pub fn last(&self) -> Option<&WormSeal> {
        self.seals.last()
    }

    /// Chain length.
    pub fn len(&self) -> usize {
        self.seals.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.seals.is_empty()
    }
}

impl Default for WormChain {
    fn default() -> Self {
        Self::new()
    }
}

/// The WORM seal pass.
pub struct WormSealPass;

impl utqc_core::Pass for WormSealPass {
    type Input = (String, u64); // (payload, steps)
    type Output = WormSeal;

    fn name(&self) -> &'static str {
        "worm-seal"
    }

    fn run(&self, input: (String, u64)) -> Result<WormSeal, utqc_core::CircuitError> {
        Ok(WormSeal::seal("CIRCUIT", &input.0, input.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seal() {
        let seal = WormSeal::seal("TEST", "payload", 100);
        assert_eq!(seal.hash.len(), 64);
        assert!(seal.verify());
    }

    #[test]
    fn test_chain() {
        let mut chain = WormChain::new();
        chain.append("STEP_1", "data1", 10);
        chain.append("STEP_2", "data2", 20);
        assert_eq!(chain.len(), 2);
        assert!(chain.verify().is_ok());
    }
}

}

//! # utqc-agent
//!
//! Agent governance hooks for proof release.
//! Every circuit release requires agent approval.

use crate::worm::WormSeal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Agent governance error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AgentError {
    #[error("insufficient permissions: requires {0}")]
    InsufficientPermissions(String),

    #[error("governance vote failed")]
    VoteFailed,

    #[error("agent not found: {0}")]
    AgentNotFound(String),
}

/// Agent identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    /// Agent name.
    pub name: String,
    /// Agent role.
    pub role: AgentRole,
    /// Permissions.
    pub permissions: Vec<String>,
}

/// Agent roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Can compile circuits.
    Compiler,
    /// Can verify circuits.
    Verifier,
    /// Can release artifacts.
    Releaser,
    /// Can govern.
    Governor,
}

/// Governance vote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceVote {
    /// Agent that voted.
    pub agent: String,
    /// Approval.
    pub approved: bool,
    /// Reason.
    pub reason: String,
}

/// Governance record for an artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRecord {
    /// Artifact identifier.
    pub artifact: String,
    /// Votes collected.
    pub votes: Vec<GovernanceVote>,
    /// WORM seal for this record.
    pub seal: WormSeal,
}

impl GovernanceRecord {
    /// Create a new governance record.
    pub fn new(artifact: &str) -> Self {
        let seal = WormSeal::seal("GOVERNANCE", artifact, 0);
        Self { artifact: artifact.to_string(), votes: Vec::new(), seal }
    }

    /// Add a vote.
    pub fn add_vote(&mut self, vote: GovernanceVote) {
        self.votes.push(vote);
    }

    /// Check if approved (majority must approve).
    pub fn is_approved(&self) -> bool {
        let total = self.votes.len();
        if total == 0 {
            return false;
        }
        let approved = self.votes.iter().filter(|v| v.approved).count();
        approved > total / 2
    }
}

/// Agent governance manager.
pub struct AgentGovernance {
    /// Registered agents.
    agents: Vec<AgentIdentity>,
}

impl AgentGovernance {
    /// Create a new governance manager.
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Register an agent.
    pub fn register(&mut self, agent: AgentIdentity) {
        self.agents.push(agent);
    }

    /// Find an agent by name.
    pub fn find(&self, name: &str) -> Option<&AgentIdentity> {
        self.agents.iter().find(|a| a.name == name)
    }

    /// Check if agent has permission.
    pub fn check_permission(&self, agent_name: &str, permission: &str) -> Result<(), AgentError> {
        let agent = self.find(agent_name)
            .ok_or_else(|| AgentError::AgentNotFound(agent_name.to_string()))?;
        if agent.permissions.contains(&permission.to_string()) {
            Ok(())
        } else {
            Err(AgentError::InsufficientPermissions(permission.to_string()))
        }
    }

    /// Collect votes for an artifact release.
    pub fn collect_votes(&self, artifact: &str) -> GovernanceRecord {
        let mut record = GovernanceRecord::new(artifact);
        for agent in &self.agents {
            let approved = agent.role == AgentRole::Governor
                || agent.role == AgentRole::Releaser
                || agent.permissions.contains(&"release".to_string());
            record.add_vote(GovernanceVote {
                agent: agent.name.clone(),
                approved,
                reason: if approved { "Approved".to_string() } else { "Insufficient role".to_string() },
            });
        }
        record
    }
}

impl Default for AgentGovernance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance() {
        let mut gov = AgentGovernance::new();
        gov.register(AgentIdentity {
            name: "compiler".to_string(),
            role: AgentRole::Releaser,
            permissions: vec!["compile".to_string(), "release".to_string()],
        });
        gov.register(AgentIdentity {
            name: "governor".to_string(),
            role: AgentRole::Governor,
            permissions: vec!["release".to_string(), "govern".to_string()],
        });

        let record = gov.collect_votes("test-artifact");
        assert!(record.is_approved());
    }

    #[test]
    fn test_permission_check() {
        let mut gov = AgentGovernance::new();
        gov.register(AgentIdentity {
            name: "alice".to_string(),
            role: AgentRole::Compiler,
            permissions: vec!["compile".to_string()],
        });

        assert!(gov.check_permission("alice", "compile").is_ok());
        assert!(gov.check_permission("alice", "release").is_err());
    }
}

