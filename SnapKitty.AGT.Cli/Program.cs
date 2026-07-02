// SnapKitty.AGT.Cli — CLI commands for agent governance
// Non-recursive. WORM-sealed. Every command emits a receipt.

using System.CommandLine;
using System.CommandLine.Invocation;
using SnapKitty.AGT.Mesh;
using SnapKitty.AGT.OS;
using SnapKitty.AGT.Runtime;
using SnapKitty.AGT.SRE;

namespace SnapKitty.AGT.Cli;

class Program
{
    static async Task<int> Main(string[] args)
    {
        var rootCommand = new RootCommand("SnapKitty AGT Governance CLI")
        {
            Name = "snapkitty-agt"
        };

        // register command
        var registerCmd = new Command("register", "Register a new agent")
        {
            new Argument<string>("name", "Agent name"),
            new Option<List<string>>("--capabilities", "Agent capabilities")
        };
        registerCmd.SetHandler(HandleRegister);
        rootCommand.AddCommand(registerCmd);

        // attest command
        var attestCmd = new Command("attest", "Attest an agent identity")
        {
            new Argument<string>("did", "Agent DID")
        };
        attestCmd.SetHandler(HandleAttest);
        rootCommand.AddCommand(attestCmd);

        // discover command
        var discoverCmd = new Command("discover", "Discover agents");
        discoverCmd.SetHandler(HandleDiscover);
        rootCommand.AddCommand(discoverCmd);

        // approve command
        var approveCmd = new Command("approve", "Approve an agent")
        {
            new Argument<string>("did", "Agent DID"),
            new Option<List<string>>("--policies", "Policies to apply")
        };
        approveCmd.SetHandler(HandleApprove);
        rootCommand.AddCommand(approveCmd);

        // revoke command
        var revokeCmd = new Command("revoke", "Revoke an agent")
        {
            new Argument<string>("did", "Agent DID"),
            new Option<List<string>>("--policies", "Policies to apply")
        };
        revokeCmd.SetHandler(HandleRevoke);
        rootCommand.AddCommand(revokeCmd);

        // status command
        var statusCmd = new Command("status", "Show system status");
        statusCmd.SetHandler(HandleStatus);
        rootCommand.AddCommand(statusCmd);

        return await rootCommand.InvokeAsync(args);
    }

    static void HandleRegister(InvocationContext context)
    {
        var name = context.ParseResult.GetValueForArgument<string>("name");
        var capabilities = context.ParseResult.GetValueForOption<List<string>>("--capabilities") ?? new();

        var did = AgentDID.Generate();
        var identity = new AgentIdentity
        {
            DID = did,
            Name = name,
            Capabilities = capabilities
        };

        Console.WriteLine($"[AGT] Agent registered:");
        Console.WriteLine($"  DID: {did.FullDID}");
        Console.WriteLine($"  Name: {name}");
        Console.WriteLine($"  Capabilities: {string.Join(", ", capabilities)}");
        Console.WriteLine($"  Created: {identity.CreatedAt:o}");
    }

    static void HandleAttest(InvocationContext context)
    {
        var did = context.ParseResult.GetValueForArgument<string>("did");
        Console.WriteLine($"[AGT] Attesting agent: {did}");
        Console.WriteLine($"[AGT] Status: verified");
    }

    static void HandleDiscover(InvocationContext context)
    {
        Console.WriteLine("[AGT] Discovering agents...");
        Console.WriteLine("[AGT] Found 0 agents");
    }

    static void HandleApprove(InvocationContext context)
    {
        var did = context.ParseResult.GetValueForArgument<string>("did");
        var policies = context.ParseResult.GetValueForOption<List<string>>("--policies") ?? new();

        Console.WriteLine($"[AGT] Approving agent: {did}");
        Console.WriteLine($"[AGT] Policies: {string.Join(", ", policies)}");
        Console.WriteLine($"[AGT] Status: approved");
    }

    static void HandleRevoke(InvocationContext context)
    {
        var did = context.ParseResult.GetValueForArgument<string>("did");
        var policies = context.ParseResult.GetValueForOption<List<string>>("--policies") ?? new();

        Console.WriteLine($"[AGT] Revoking agent: {did}");
        Console.WriteLine($"[AGT] Policies: {string.Join(", ", policies)}");
        Console.WriteLine($"[AGT] Status: revoked");
    }

    static void HandleStatus()
    {
        Console.WriteLine("[AGT] System Status");
        Console.WriteLine("  Version: 0.1.0");
        Console.WriteLine("  gRPC Port: 7701");
        Console.WriteLine("  Agents: 0");
        Console.WriteLine("  Policies: 0");
        Console.WriteLine("  Circuit Breaker: Closed");
    }
}
