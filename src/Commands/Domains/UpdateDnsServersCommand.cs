using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class UpdateDnsServersCommand : AsyncCommand<UpdateDnsServersCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to update")]
        public required string Domain { get; init; }

        [CommandOption("--nameserver <HOST>")]
        [Description("A name server; repeat the option, at least twice")]
        public string[] Nameservers { get; init; } = [];
    }

    public override ValidationResult Validate(CommandContext context, Settings settings) =>
        settings.Nameservers.Length < 2
            ? ValidationResult.Error("At least two --nameserver values are required.")
            : ValidationResult.Success();

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("updateDNSServers", settings.Domain, settings.Nameservers);
        LoopiaStatus.EnsureOk(result, "updateDNSServers");
        AnsiConsole.MarkupLine($"[green]Name servers for {settings.Domain} updated.[/]");
        return 0;
    }
}
