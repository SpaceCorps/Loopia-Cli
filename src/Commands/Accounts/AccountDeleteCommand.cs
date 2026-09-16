using System.ComponentModel;
using Loopia.Console.Infrastructure.Accounts;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Accounts;

public sealed class AccountDeleteCommand : Command<AccountDeleteCommand.Settings>
{
    public sealed class Settings : CommandSettings
    {
        [CommandArgument(0, "<NAME>")]
        [Description("Name of the stored account to delete")]
        public required string Name { get; init; }

        [CommandOption("-y|--yes")]
        [Description("Delete without asking for confirmation")]
        public bool Yes { get; init; }
    }

    public override int Execute(CommandContext context, Settings settings)
    {
        var file = AccountStore.Load();
        var account = file.Find(settings.Name);

        if (account is null)
        {
            AnsiConsole.MarkupLineInterpolated($"[red]No account named '{settings.Name}'.[/]");
            return 1;
        }

        if (!settings.Yes && !Prompts.Confirm($"Delete account '{account.Name}' ({account.Username})?"))
        {
            AnsiConsole.MarkupLine("[yellow]Cancelled.[/]");
            return 1;
        }

        file.Accounts.Remove(account);
        if (string.Equals(file.DefaultAccount, account.Name, StringComparison.OrdinalIgnoreCase))
            file.DefaultAccount = null;

        AccountStore.Save(file);
        AnsiConsole.MarkupLineInterpolated($"Deleted account [green]{account.Name}[/].");
        return 0;
    }
}
