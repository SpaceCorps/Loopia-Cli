using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class CheckDomainCommand : AsyncCommand<CheckDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to check (ACE-coded for IDN domains)")]
        public required string Domain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var status = LoopiaStatus.Read(await client.CallGlobalAsync("domainIsFree", settings.Domain));
        if (status == LoopiaStatus.Ok)
        {
            AnsiConsole.MarkupLine($"[green]{settings.Domain} is available.[/]");
            return 0;
        }

        AnsiConsole.MarkupLine($"[yellow]{settings.Domain} is not available ({status}).[/]");
        return 1;
    }
}
