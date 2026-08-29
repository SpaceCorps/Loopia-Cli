using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Subdomains;

public sealed class ListSubdomainsCommand : AsyncCommand<ListSubdomainsCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name whose subdomains to list")]
        public required string Domain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getSubdomains", settings.Domain);
        YamlOutput.Write(result);
        return 0;
    }
}
