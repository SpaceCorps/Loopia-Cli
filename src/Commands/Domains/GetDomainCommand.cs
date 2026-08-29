using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Domains;

public sealed class GetDomainCommand : AsyncCommand<GetDomainCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("The domain name to look up (ACE-coded for IDN domains)")]
        public required string Domain { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getDomain", settings.Domain);
        YamlOutput.Write(result);
        return 0;
    }
}
