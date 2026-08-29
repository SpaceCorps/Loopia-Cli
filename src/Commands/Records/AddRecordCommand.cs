using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Records;

public sealed class AddRecordCommand : AsyncCommand<AddRecordCommand.Settings>
{
    public sealed class Settings : RecordSettings
    {
        [CommandOption("--type <TYPE>")]
        [Description("Record type, e.g. A, AAAA, CNAME, MX, TXT, NS, SRV, CAA")]
        public required string Type { get; init; }

        [CommandOption("--rdata <RDATA>")]
        [Description("Record data, e.g. an IP address or a host name")]
        public required string Rdata { get; init; }

        [CommandOption("--ttl <SECONDS>")]
        [Description("Time to live in seconds (default: 3600)")]
        public int Ttl { get; init; } = 3600;

        [CommandOption("--priority <PRIORITY>")]
        [Description("Priority, used by MX and SRV records (default: 0)")]
        public int Priority { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var record = ZoneRecord.Build(settings.Type, settings.Rdata, settings.Ttl, settings.Priority, recordId: 0);
        var result = await client.CallAsync("addZoneRecord", settings.Domain, settings.Subdomain, record);
        LoopiaStatus.EnsureOk(result, "addZoneRecord");
        AnsiConsole.MarkupLine($"[green]{settings.Type} record added to {settings.Subdomain}.{settings.Domain}.[/]");
        return 0;
    }
}
