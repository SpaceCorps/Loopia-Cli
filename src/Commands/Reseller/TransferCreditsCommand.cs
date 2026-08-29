using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Reseller;

public sealed class TransferCreditsCommand : AsyncCommand<TransferCreditsCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--from <CUSTOMER_NUMBER>")]
        [Description("Customer number of the sending account")]
        public required string From { get; init; }

        [CommandOption("--to <CUSTOMER_NUMBER>")]
        [Description("Customer number of the receiving account")]
        public required string To { get; init; }

        [CommandOption("--amount <AMOUNT>")]
        [Description("The amount to transfer")]
        public required double Amount { get; init; }

        [CommandOption("--currency <CURRENCY>")]
        [Description("Three-letter ISO 4217 currency code, SEK or NOK (default: SEK)")]
        public string Currency { get; init; } = "SEK";
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallGlobalAsync("transferCreditsByCurrency",
            settings.From, settings.To, settings.Amount, settings.Currency);
        LoopiaStatus.EnsureOk(result, "transferCreditsByCurrency");
        AnsiConsole.MarkupLine($"[green]Transferred {settings.Amount} {settings.Currency} from {settings.From} to {settings.To}.[/]");
        return 0;
    }
}
