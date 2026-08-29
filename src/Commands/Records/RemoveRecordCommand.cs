using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Records;

public sealed class RemoveRecordCommand : AsyncCommand<RemoveRecordCommand.Settings>
{
    public sealed class Settings : RecordSettings
    {
        [CommandOption("--record-id <ID>")]
        [Description("The ID of the record to remove, as returned by 'records list'")]
        public required int RecordId { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallAsync("removeZoneRecord", settings.Domain, settings.Subdomain, settings.RecordId);
        LoopiaStatus.EnsureOk(result, "removeZoneRecord");
        AnsiConsole.MarkupLine($"[green]Record {settings.RecordId} removed.[/]");
        return 0;
    }
}
