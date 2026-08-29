using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Billing;

public sealed class PayInvoiceCommand : AsyncCommand<PayInvoiceCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--reference-no <REFERENCE>")]
        [Description("The reference number of the invoice to pay")]
        public required string ReferenceNo { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("payInvoiceUsingCredits", settings.ReferenceNo);
        LoopiaStatus.EnsureOk(result, "payInvoiceUsingCredits");
        AnsiConsole.MarkupLine($"[green]Invoice {settings.ReferenceNo} paid with LoopiaPrePAID credits.[/]");
        return 0;
    }
}
