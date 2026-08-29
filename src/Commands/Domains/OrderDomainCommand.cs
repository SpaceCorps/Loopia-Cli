using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class OrderDomainCommand : AsyncCommand<OrderDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to register (ACE-coded for IDN domains)")]
        public required string Domain { get; init; }

        [CommandOption("--accept-terms")]
        [Description("Confirm that the account owner has accepted the registration terms and conditions")]
        public bool AcceptTerms { get; init; }
    }

    public override ValidationResult Validate(CommandContext context, Settings settings) =>
        settings.AcceptTerms
            ? ValidationResult.Success()
            : ValidationResult.Error("Loopia rejects orders without accepted terms; pass --accept-terms.");

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("orderDomain", settings.Domain, true);
        LoopiaStatus.EnsureOk(result, "orderDomain");
        AnsiConsole.MarkupLine($"[green]Domain {settings.Domain} ordered.[/]");
        return 0;
    }
}
