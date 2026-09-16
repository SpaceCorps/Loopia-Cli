using Spectre.Console;

namespace Loopia.Console.Commands.Accounts;

/// <summary>
/// Prompt helpers that also work when stdin is piped, which Spectre's own prompts refuse to do.
/// </summary>
internal static class Prompts
{
    public static string Ask(string markup, bool secret)
    {
        if (!System.Console.IsInputRedirected)
        {
            var prompt = new TextPrompt<string>(markup).AllowEmpty();
            if (secret)
                prompt.Secret();
            return AnsiConsole.Prompt(prompt);
        }

        AnsiConsole.Markup(markup + " ");
        var line = System.Console.In.ReadLine();
        AnsiConsole.WriteLine();
        return line?.Trim() ?? string.Empty;
    }

    public static bool Confirm(string question)
    {
        if (!System.Console.IsInputRedirected)
            return AnsiConsole.Confirm(question, defaultValue: false);

        AnsiConsole.Markup($"{question.EscapeMarkup()} [[y/N]] ");
        var line = System.Console.In.ReadLine()?.Trim();
        AnsiConsole.WriteLine();
        return line is not null
            && (line.Equals("y", StringComparison.OrdinalIgnoreCase) || line.Equals("yes", StringComparison.OrdinalIgnoreCase));
    }
}
