using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Billing;

public sealed class CreditsCommand : AsyncCommand<CreditsCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--with-vat")]
        [Description("Include VAT in the returned amount")]
        public bool WithVat { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getCreditsAmount", settings.WithVat);
        YamlOutput.Write(result);
        return 0;
    }
}
