using Loopia.Console.Infrastructure;
using Loopia.Console.Infrastructure.Accounts;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Accounts;

public sealed class AccountListCommand : Command<AccountListCommand.Settings>
{
    public sealed class Settings : CommandSettings
    {
        [CommandOption("--paths")]
        [System.ComponentModel.Description("Show where the accounts are stored and how they are encrypted")]
        public bool ShowPaths { get; init; }
    }

    public override int Execute(CommandContext context, Settings settings)
    {
        var file = AccountStore.Load();

        if (settings.ShowPaths)
        {
            AnsiConsole.MarkupLineInterpolated($"Store:      [grey]{AccountStore.FilePath}[/]");
            AnsiConsole.MarkupLineInterpolated($"Encryption: [grey]{SecretProtector.Describe()}[/]");
            AnsiConsole.WriteLine();
        }

        if (file.Accounts.Count == 0)
        {
            AnsiConsole.MarkupLine("No accounts configured. Add one with [yellow]loopia account create <name>[/].");
            return 0;
        }

        var defaultAccount = file.EffectiveDefault();

        var table = new Table().Border(TableBorder.Rounded);
        table.AddColumn("Account");
        table.AddColumn("Username");
        table.AddColumn("Customer number");
        table.AddColumn("Endpoint");

        foreach (var account in file.Accounts)
        {
            var isDefault = string.Equals(account.Name, defaultAccount, StringComparison.OrdinalIgnoreCase);
            table.AddRow(
                new Markup(isDefault ? $"[green]{account.Name.EscapeMarkup()} *[/]" : account.Name.EscapeMarkup()),
                new Markup(account.Username.EscapeMarkup()),
                new Markup((account.CustomerNumber ?? "-").EscapeMarkup()),
                new Markup((account.Endpoint ?? LoopiaClient.DefaultEndpoint).EscapeMarkup()));
        }

        AnsiConsole.Write(table);
        if (defaultAccount is not null)
            AnsiConsole.MarkupLine("[grey]* used when no --account is given[/]");
        return 0;
    }
}
