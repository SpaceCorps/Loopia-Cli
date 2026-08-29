using System.ComponentModel;
using Spectre.Console.Cli;

namespace Loopia.Console.Infrastructure;

public class ApiSettings : CommandSettings
{
    [CommandOption("--username <USERNAME>")]
    [Description("LoopiaAPI username, e.g. user@loopiaapi (or set LOOPIA_API_USERNAME env var)")]
    public string? Username { get; init; }

    [CommandOption("--password <PASSWORD>")]
    [Description("LoopiaAPI password (or set LOOPIA_API_PASSWORD env var)")]
    public string? Password { get; init; }

    [CommandOption("--customer-number <NUMBER>")]
    [Description("Customer number, resellers only (or set LOOPIA_CUSTOMER_NUMBER env var)")]
    public string? CustomerNumber { get; init; }

    [CommandOption("--endpoint <URL>")]
    [Description("XML-RPC endpoint (or set LOOPIA_API_ENDPOINT env var)")]
    public string? Endpoint { get; init; }

    public LoopiaClient CreateClient()
    {
        var username = Username ?? Environment.GetEnvironmentVariable("LOOPIA_API_USERNAME")
            ?? throw new InvalidOperationException("Username required. Use --username or set LOOPIA_API_USERNAME.");
        var password = Password ?? Environment.GetEnvironmentVariable("LOOPIA_API_PASSWORD")
            ?? throw new InvalidOperationException("Password required. Use --password or set LOOPIA_API_PASSWORD.");
        var customerNumber = CustomerNumber ?? Environment.GetEnvironmentVariable("LOOPIA_CUSTOMER_NUMBER");
        var endpoint = Endpoint ?? Environment.GetEnvironmentVariable("LOOPIA_API_ENDPOINT");
        return new LoopiaClient(username, password, customerNumber, endpoint);
    }
}
