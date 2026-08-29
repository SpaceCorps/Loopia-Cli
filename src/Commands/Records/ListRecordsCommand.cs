using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Records;

public sealed class ListRecordsCommand : AsyncCommand<RecordSettings>
{
    public override async Task<int> ExecuteAsync(CommandContext context, RecordSettings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("getZoneRecords", settings.Domain, settings.Subdomain);
        YamlOutput.Write(result);
        return 0;
    }
}
