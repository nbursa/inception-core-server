# Semantic Memory System – Technical Design (Text-Based Prototype)

## Purpose

This system models memory as a dynamic semantic network of object-centered clusters formed through text-based perceptual input. It encodes, organizes, and reclassifies knowledge through experience — meaning is not predefined but emerges from accumulated relationships, observed traits, and affective outcomes.

---

## Core Principles

### 1. Object-Centric Memory
- Each perceived entity (e.g. "knife", "bug", "table") is represented as an `ObjectCluster`.
- Clusters accumulate sensory traits, semantic tags, relations, and affect.
- Objects start as `Unknown` and become `Known` through repeated observation and interaction.

### 2. Emergent Meaning
- Meaning is a consequence of relational density and affective evaluation, not a fixed label.
- Percepts without known identity are stored with descriptors (e.g. "shiny", "cold", "sharp").
- Consequences (e.g. "pain", "pleasure") dynamically reclassify entities.

### 3. Semantic Space
- All entities are embedded into a shared high-dimensional latent space (`Vec<f32>`).
- Proximity in this space implies conceptual similarity (e.g. `knife` ↔ `scalpel`, not `mountain`).

---

## System Architecture

TextInput → Embedding → LatentGraph → SemanticEvaluator

### Modules

#### 1. TextInput
- Raw perceptual description: `"cold shiny elongated object"`
- Embedded into semantic vector: `Vec<f32>` via external model (e.g. SentenceTransformer)

#### 2. ObjectCluster
```rust
struct ObjectCluster {
  name: String,              // "x_13", "knife", etc.
  embedding: Vec<f32>,       // semantic vector
  tags: Vec<String>,         // e.g. ["cold", "metal", "sharp"]
  affect: AffectScore,       // [-1.0 to 1.0] + Affect enum
  known: bool,               // reclassified or still latent
}

3. LatentGraph

struct LatentGraph {
  clusters: HashMap<String, ObjectCluster>,
  relations: Vec<(String, String, String)>, // (source, relation, target)
}

	•	Supports similarity search, reflection, and experience-driven relation building.

4. SemanticEvaluator
	•	Assigns tags based on keywords or embedding similarity.
	•	Calculates AffectScore from tags and consequences.
	•	Triggers reclassification (unknown → known) once meaning is stable.


Affect Model

enum Affect { Pleasant, Unpleasant, Unknown }

struct AffectScore {
  value: f32,     // [-1.0 (negative) to +1.0 (positive)]
  class: Affect,  // auto-derived from value
}

Example Affect Tags

Tag	Score
“pain”	-1.0
“sharp”	-0.7
“beautiful”	+0.9
“safe”	+0.6



Workflow Example

Input: "cold shiny elongated object"
	1.	Embed text into vector.
	2.	No existing cluster matches → create new ObjectCluster:
	•	name = "x_17"
	•	tags = ["cold", "shiny", "elongated"]
	•	affect = Unknown

Later Input: "cut my finger on the object"
	1.	Find nearest cluster (x_17)
	2.	Add tag: "sharp"
	3.	Add relation: x_17 -[caused]-> pain
	4.	Affect recalculated: value = -0.85 → class = Unpleasant

Final Reclassification:
	•	x_17 becomes knife
	•	known = true
	•	Now tagged as: ["metal", "tool", "sharp", "dangerous"]
	•	Can relate to: "fork", "scalpel", "weapon"


Reflection

fn reflect(object: &str) -> Vec<(String, f32)> {
  // returns semantically closest clusters
}

→ reflect("knife") → ["scalpel", "blade", "danger", "tool"]


Classification and Learning
	•	Objects are reclassified when:
	•	Tag density exceeds threshold
	•	Affect score becomes stable
	•	Named association (e.g. “this is a knife”) is added


Long-Term Goals
	•	Extend from text-only to multi-modal perception (PerceptualInput)
	•	Build episodic chains (memory of sequences, not isolated objects)
	•	Enable reflective reasoning and generalization
	•	Use weighted decay and reinforcement to simulate forgetting and attention


Summary
	•	Objects start as unknowns with observed traits
	•	Semantic relations and consequences accumulate over time
	•	Meaning and affect emerge, not defined upfront
	•	Text input is enough to prototype full self-growing memory system

---
