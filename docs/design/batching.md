## Scheduler Design.

```mermaid
flowchart TD
    A[Client Sends Inference Requests] --> B[HTTP/gRPC API Layer]
    B --> C[Model Manager]

    subgraph "Dynamic Batching Scheduler"
        C --> D{Batch Window Open?}
        D -- Yes --> E[Accumulate Requests]
        E --> F{Preferred Batch Size Reached?}
        F -- Yes --> G[Dispatch Batch to Model Backend]
        F -- No --> H{Timeout?}
        H -- Yes --> G
        H -- No --> E
        D -- No --> C
    end

    G --> I[Model Execution on GPU/CPU]
    I --> J[Return Batched Results]
    J --> K[Client Receives Response]
```
