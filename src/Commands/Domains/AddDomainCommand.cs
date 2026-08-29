using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class AddDomainCommand : AsyncCommand<AddDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("An already registered domain name to add to the account")]
        public required string Domain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("addDomain", settings.Domain);
        LoopiaStatus.EnsureOk(result, "addDomain");
        AnsiConsole.MarkupLine($"[green]Domain {settings.Domain} added.[/]");
        return 0;
    }
}
