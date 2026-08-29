using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Records;

/// <summary>
/// Zone records are always addressed by domain plus subdomain; "@" is the domain itself.
/// </summary>
public class RecordSettings : ApiSettings
{
    [CommandOption("--domain <DOMAIN>")]
    [Description("The domain name holding the zone")]
    public required string Domain { get; init; }

    [CommandOption("--subdomain <SUBDOMAIN>")]
    [Description("The subdomain, or @ for the domain itself (default: @)")]
    public string Subdomain { get; init; } = "@";
}
