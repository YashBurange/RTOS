# RTOS

- scheduler
- timing system
- synchronization / IPC
- interrupt handling
- resource control
## System Constraints

This RTOS is governed by the following core constraints:

### Time
- All operations must have bounded execution time
- Scheduling must be deterministic under identical conditions
- Deadlines must be observable and enforceable

### Safety
- No undefined memory behavior is permitted
- Failures must be contained and recoverable
- Invalid system states must be unrepresentable or rejected

### Isolation
- All access must be governed by explicit capabilities
- Shared state must be controlled and bounded

### Resources
- Memory and CPU usage must be bounded
- System must handle overload through defined policies

### Correctness
- Only valid actions may execute
- State must remain consistent at all times

### Observability
- All critical events and faults must be traceable
- System behavior must be inspectable

### Modularity
- Core kernel must remain policy-agnostic
- Components must be replaceable without affecting core behavior

### Portability
- Hardware-specific logic must be isolated
- System must support multiple architectures
  
## Resources
---
- https://www.researchgate.net/publication/322181411_A_Review_of_the_Scopes_and_Challenges_of_the_Modern_Real-Time_Operating_Systems
    - https://sci-hub.ru/10.4018/IJERTCS.2018010104 
