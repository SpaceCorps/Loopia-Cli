using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Billing;

public sealed class GetInvoiceCommand : AsyncCommand<GetInvoiceCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--reference-no <REFERENCE>")]
        [Description("The invoice reference number")]
        public required string ReferenceNo { get; init; }

        [CommandOption("--with-vat")]
        [Description("Include VAT in the returned amounts")]
        public bool WithVat { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getInvoice", settings.ReferenceNo, settings.WithVat);
        YamlOutput.Write(result);
        return 0;
    }
}
