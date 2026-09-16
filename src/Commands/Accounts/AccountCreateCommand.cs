using System.ComponentModel;
using Loopia.Console.Infrastructure.Accounts;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Accounts;

public sealed class AccountCreateCommand : Command<AccountCreateCommand.Settings>
{
    public sealed class Settings : CommandSettings
    {
        [CommandArgument(0, "<NAME>")]
        [Description("Name you will pass to --account, e.g. ivy")]
        public required string Name { get; init; }

        [CommandOption("--username <USERNAME>")]
        [Description("LoopiaAPI username, e.g. user@loopiaapi")]
        public string? Username { get; init; }

        [CommandOption("--password <PASSWORD>")]
        [Description("LoopiaAPI password; prompted for without echo when omitted")]
        public string? Password { get; init; }

        [CommandOption("--customer-number <NUMBER>")]
        [Description("Customer number, resellers only")]
        public string? CustomerNumber { get; init; }

        [CommandOption("--endpoint <URL>")]
        [Description("XML-RPC endpoint, when it differs from the Loopia default")]
        public string? Endpoint { get; init; }

        [CommandOption("--default")]
        [Description("Use this account when no --account is given")]
        public bool Default { get; init; }

        [CommandOption("--force")]
        [Description("Overwrite an account of the same name without asking")]
        public bool Force { get; init; }
    }

    public override ValidationResult Validate(CommandContext context, Settings settings) =>
        string.IsNullOrWhiteSpace(settings.Name)
            ? ValidationResult.Error("The account name cannot be empty.")
            : ValidationResult.Success();

    public override int Execute(CommandContext context, Settings settings)
    {
        var file = AccountStore.Load();
        var existing = file.Find(settings.Name);

        if (existing is not null && !settings.Force
            && !Prompts.Confirm($"Account '{settings.Name}' already exists. Replace it?"))
        {
            AnsiConsole.MarkupLine("[yellow]Cancelled.[/]");
            return 1;
        }

        var username = settings.Username ?? Prompts.Ask("LoopiaAPI [green]username[/]:", secret: false);
        var password = settings.Password ?? Prompts.Ask("LoopiaAPI [green]password[/]:", secret: true);

        if (string.IsNullOrWhiteSpace(username) || string.IsNullOrWhiteSpace(password))
        {
            AnsiConsole.MarkupLine("[red]Username and password are both required.[/]");
            return 1;
        }

        var account = new StoredAccount
        {
            Name = settings.Name,
            Username = username,
            ProtectedPassword = SecretProtector.Protect(password),
            CustomerNumber = string.IsNullOrWhiteSpace(settings.CustomerNumber) ? null : settings.CustomerNumber,
            Endpoint = string.IsNullOrWhiteSpace(settings.Endpoint) ? null : settings.Endpoint
        };

        if (existing is not null)
            file.Accounts.Remove(existing);
        file.Accounts.Add(account);

        if (settings.Default)
            file.DefaultAccount = account.Name;

        AccountStore.Save(file);

        AnsiConsole.MarkupLineInterpolated(
            $"Saved account [green]{account.Name}[/] to [grey]{AccountStore.FilePath}[/] (password encrypted with {SecretProtector.Describe()}).");
        if (string.Equals(file.EffectiveDefault(), account.Name, StringComparison.OrdinalIgnoreCase))
            AnsiConsole.MarkupLine("[grey]It is used when no --account is given.[/]");
        return 0;
    }
}
