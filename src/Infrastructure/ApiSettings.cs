using System.ComponentModel;
using Loopia.Console.Infrastructure.Accounts;
using Spectre.Console.Cli;

namespace Loopia.Console.Infrastructure;

public class ApiSettings : CommandSettings
{
    [CommandOption("-a|--account <NAME>")]
    [Description("Use the credentials of a stored account (or set LOOPIA_ACCOUNT env var)")]
    public string? Account { get; init; }

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

    /// <summary>
    /// Builds a client from, in order of precedence: the explicit options, the selected
    /// stored account, then the environment variables.
    /// </summary>
    public LoopiaClient CreateClient()
    {
        var account = AccountStore.Resolve(Account);

        var username = Username
            ?? account?.Username
            ?? Environment.GetEnvironmentVariable("LOOPIA_API_USERNAME")
            ?? throw new InvalidOperationException(
                "Username required. Use --account, --username or set LOOPIA_API_USERNAME.");

        var password = Password
            ?? (account is null ? null : SecretProtector.Unprotect(account.ProtectedPassword))
            ?? Environment.GetEnvironmentVariable("LOOPIA_API_PASSWORD")
            ?? throw new InvalidOperationException(
                "Password required. Use --account, --password or set LOOPIA_API_PASSWORD.");

        var customerNumber = CustomerNumber
            ?? account?.CustomerNumber
            ?? Environment.GetEnvironmentVariable("LOOPIA_CUSTOMER_NUMBER");

        var endpoint = Endpoint
            ?? account?.Endpoint
            ?? Environment.GetEnvironmentVariable("LOOPIA_API_ENDPOINT");

        return new LoopiaClient(username, password, customerNumber, endpoint);
    }
}
