using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Subdomains;

public sealed class AddSubdomainCommand : AsyncCommand<AddSubdomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to attach the subdomain to")]
        public required string Domain { get; init; }

        [CommandOption("--subdomain <SUBDOMAIN>")]
        [Description("The subdomain to add, e.g. www")]
        public required string Subdomain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("addSubdomain", settings.Domain, settings.Subdomain);
        LoopiaStatus.EnsureOk(result, "addSubdomain");
        AnsiConsole.MarkupLine($"[green]Subdomain {settings.Subdomain}.{settings.Domain} added.[/]");
        return 0;
    }
}
