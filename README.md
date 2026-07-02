# sovereign-agt

> C# governance stack for the SnapKitty ecosystem.
> Mesh. Runtime. OS. Observability. gRPC.

[![License: Sovereign Source](https://img.shields.io/badge/License-Sovereign%20Source-blue.svg)](SOVEREIGN.md)
[![C#](https://img.shields.io/badge/C%23-12-purple.svg)](https://dotnet.microsoft.com/)
[![.NET](https://img.shields.io/badge/.NET-8.0-blue.svg)](https://dotnet.microsoft.com/)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SNAPKITTY AGT GOVERNANCE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    gRPC Interface                         │    │
│  │                    (port 7701)                            │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│          ┌────────────────┼────────────────┐                    │
│          ▼                ▼                ▼                    │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────┐       │
│  │  AGT.Mesh    │ │ AGT.Runtime  │ │    AGT.OS        │       │
│  │  (service    │ │  (execution  │ │   (process       │       │
│  │   discovery) │ │   engine)    │ │    control)      │       │
│  └──────────────┘ └──────────────┘ └──────────────────┘       │
│          │                │                │                    │
│          └────────────────┼────────────────┘                    │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    AGT.SRE                               │    │
│  │          (Site Reliability Engineering)                  │    │
│  │     Health checks / Metrics / Alerts / Dashboards        │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                    AGT.Cli                                │    │
│  │          (Command-line interface)                         │    │
│  │     status / start / stop / logs / config                │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Solution Structure

```
SnapKitty.AGT/
├── AGT.Mesh/              Service discovery and load balancing
│   ├── MeshService.cs     Core mesh operations
│   ├── PeerDiscovery.cs   Peer discovery protocol
│   └── LoadBalancer.cs    Round-robin load balancing
│
├── AGT.Runtime/           Execution engine
│   ├── RuntimeService.cs  Core runtime operations
│   ├── TaskScheduler.cs   Non-recursive task scheduling
│   └── ProcessManager.cs  Process lifecycle management
│
├── AGT.OS/                Process control
│   ├── OSService.cs       Core OS operations
│   ├── ProcessControl.cs  Process start/stop/signal
│   └── ResourceMonitor.cs CPU/memory/disk monitoring
│
├── AGT.SRE/               Site reliability engineering
│   ├── SREService.cs      Core SRE operations
│   ├── HealthCheck.cs     Health check endpoints
│   ├── Metrics.cs         Metrics collection
│   └── Alerting.cs        Alert management
│
├── AGT.Grpc/              gRPC server (port 7701)
│   ├── GrpcServer.cs      Server configuration
│   ├── Proto/             Protocol Buffers
│   └── Services/          gRPC service implementations
│
├── AGT.Cli/               Command-line interface
│   ├── Program.cs         Entry point
│   ├── Commands/          CLI commands
│   └── Output/            Output formatting
│
└── AGT.Tests/             Unit tests
    ├── MeshTests.cs
    ├── RuntimeTests.cs
    ├── OSTests.cs
    └── SRETests.cs
```

## Quick Start

### Build

```bash
dotnet restore
dotnet build
```

### Run gRPC Server

```bash
dotnet run --project AGT.Grpc
# Server listening on port 7701
```

### CLI Usage

```bash
# Check status
dotnet run --project AGT.Cli -- status

# Start service
dotnet run --project AGT.Cli -- start --service mesh

# Stop service
dotnet run --project AGT.Cli -- stop --service runtime

# View logs
dotnet run --project AGT.Cli -- logs --service os --tail

# Health check
dotnet run --project AGT.Cli -- health
```

### gRPC Client

```csharp
using Grpc.Net.Client;

var channel = GrpcChannel.ForAddress("http://localhost:7701");
var client = new AGT.Mesh.Mesh.MeshClient(channel);

// Discover peers
var peers = await client.DiscoverPeersAsync(new DiscoverPeersRequest());
foreach (var peer in peers.Peers) {
    Console.WriteLine($"Peer: {peer.Id} at {peer.Address}");
}
```

## Interactive Demo

```bash
# Demo 1: Service mesh status
$ dotnet run --project AGT.Cli -- status
┌─────────────────────────────────────────┐
│           AGT Mesh Status               │
├─────────────────────────────────────────┤
│ Service     │ Status  │ Peers │ Uptime  │
├─────────────────────────────────────────┤
│ mesh        │ ✓ UP    │   3   │ 2h 15m  │
│ runtime     │ ✓ UP    │   3   │ 2h 15m  │
│ os          │ ✓ UP    │   3   │ 2h 15m  │
│ sre         │ ✓ UP    │   3   │ 2h 15m  │
└─────────────────────────────────────────┘

# Demo 2: Health check
$ dotnet run --project AGT.Cli -- health
{
  "status": "healthy",
  "services": {
    "mesh": "healthy",
    "runtime": "healthy",
    "os": "healthy",
    "sre": "healthy"
  },
  "timestamp": "2025-01-01T00:00:00Z"
}

# Demo 3: Process control
$ dotnet run --project AGT.Cli -- start --service runtime
Starting service: runtime... ✓

$ dotnet run --project AGT.Cli -- stop --service runtime
Stopping service: runtime... ✓
```

## gRPC Services

| Service | Port | Description |
|---------|------|-------------|
| AGT.Mesh | 7701 | Service discovery, load balancing |
| AGT.Runtime | 7701 | Task scheduling, process management |
| AGT.OS | 7701 | Process control, resource monitoring |
| AGT.SRE | 7701 | Health checks, metrics, alerting |

## Invariants

| Invariant | Description |
|-----------|-------------|
| **No Recursion** | All task scheduling is iterative |
| **Typed Contracts** | All gRPC messages are strongly typed |
| **Deterministic** | Same config → same behavior |
| **Fail-Closed** | Any error terminates processing |
| **Observable** | Every state change is logged |

## Testing

```bash
# Run all tests
dotnet test

# Run with coverage
dotnet test /p:CollectCoverage=true

# Run specific test class
dotnet test --filter "ClassName=AGT.Tests.MeshTests"
```

## License

Sovereign Source License — see [SOVEREIGN.md](SOVEREIGN.md)

---

```
SNAPKITTY-AGT-CSHARP-001
Mesh. Runtime. OS. SRE. gRPC.
Same service. Same state.
No recursion. No borrowed thesis.
```
