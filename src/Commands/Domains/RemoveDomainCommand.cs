using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class RemoveDomainCommand : AsyncCommand<RemoveDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to remove")]
        public required string Domain { get; init; }

        [CommandOption("--deactivate")]
        [Description("Deactivate now and remove on the due date instead of removing immediately")]
        public bool Deactivate { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("removeDomain", settings.Domain, settings.Deactivate);
        LoopiaStatus.EnsureOk(result, "removeDomain");
        AnsiConsole.MarkupLine(settings.Deactivate
            ? $"[green]Domain {settings.Domain} deactivated; it will be removed on its due date.[/]"
            : $"[green]Domain {settings.Domain} removed.[/]");
        return 0;
    }
}
