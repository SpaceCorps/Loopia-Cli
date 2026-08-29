using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Subdomains;

public sealed class RemoveSubdomainCommand : AsyncCommand<RemoveSubdomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name containing the subdomain")]
        public required string Domain { get; init; }

        [CommandOption("--subdomain <SUBDOMAIN>")]
        [Description("The subdomain to remove")]
        public required string Subdomain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("removeSubdomain", settings.Domain, settings.Subdomain);
        LoopiaStatus.EnsureOk(result, "removeSubdomain");
        AnsiConsole.MarkupLine($"[green]Subdomain {settings.Subdomain}.{settings.Domain} removed.[/]");
        return 0;
    }
}
