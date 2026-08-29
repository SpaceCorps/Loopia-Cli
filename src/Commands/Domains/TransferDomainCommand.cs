using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class TransferDomainCommand : AsyncCommand<TransferDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to transfer to Loopia")]
        public required string Domain { get; init; }

        [CommandOption("--auth-code <CODE>")]
        [Description("The auth code from the current registrar")]
        public required string AuthCode { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("transferDomain", settings.Domain, settings.AuthCode);
        LoopiaStatus.EnsureOk(result, "transferDomain");
        AnsiConsole.MarkupLine($"[green]Transfer of {settings.Domain} started.[/]");
        return 0;
    }
}
