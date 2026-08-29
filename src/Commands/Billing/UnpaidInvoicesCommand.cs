using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Billing;

public sealed class UnpaidInvoicesCommand : AsyncCommand<UnpaidInvoicesCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--with-vat")]
        [Description("Include VAT in the returned amounts")]
        public bool WithVat { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getUnpaidInvoices", settings.WithVat);
        YamlOutput.Write(result);
        return 0;
    }
}
