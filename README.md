# PoC - Flow Orchestrator Pattern

![alt flow-orchestrator-pattern](flow.drawio.png?raw=true "flow-orchestrator-pattern")


It seeks to establish a pattern to organize this diagram of flows and tasks with the following conditions:

- Infinite recursion must be guaranteed without Stack Overflow.

- The pattern should be optimized for the highest speed and lowest resource usage.

- To use the pattern it will only be necessary to know the list of logical procedures organized in groups of flows and tasks connected by conditional arrows. As represented in the diagram.

- Each task can modify a global context from a mutable variable that will be represented with the most appropriate data type for the program. (String, HashMap, Vec)