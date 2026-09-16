using System.Text.Json;

namespace Loopia.Console.Infrastructure.Accounts;

/// <summary>
/// Reads and writes the named accounts kept in the user's config directory.
/// </summary>
public static class AccountStore
{
    private static readonly JsonSerializerOptions JsonOptions = new() { WriteIndented = true };

    public static string DirectoryPath
    {
        get
        {
            var overridden = Environment.GetEnvironmentVariable("LOOPIA_CONFIG_DIR");
            if (!string.IsNullOrWhiteSpace(overridden))
                return overridden;

            var appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
            if (string.IsNullOrWhiteSpace(appData))
                appData = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".config");
            return Path.Combine(appData, "loopia");
        }
    }

    public static string FilePath => Path.Combine(DirectoryPath, "accounts.json");

    public static string KeyFilePath => Path.Combine(DirectoryPath, "accounts.key");

    public static AccountFile Load()
    {
        if (!File.Exists(FilePath))
            return new AccountFile();

        try
        {
            return JsonSerializer.Deserialize<AccountFile>(File.ReadAllText(FilePath)) ?? new AccountFile();
        }
        catch (JsonException ex)
        {
            throw new InvalidOperationException($"'{FilePath}' is not valid JSON: {ex.Message}");
        }
    }

    public static void Save(AccountFile file)
    {
        file.Accounts.Sort(static (a, b) => string.Compare(a.Name, b.Name, StringComparison.OrdinalIgnoreCase));
        Directory.CreateDirectory(DirectoryPath);
        File.WriteAllText(FilePath, JsonSerializer.Serialize(file, JsonOptions));
        SecretProtector.RestrictToOwner(FilePath);
    }

    public static StoredAccount? Find(this AccountFile file, string name) =>
        file.Accounts.FirstOrDefault(a => string.Equals(a.Name, name, StringComparison.OrdinalIgnoreCase));

    /// <summary>
    /// The account used when none is named: the configured default, or the only account when
    /// there is just one, so a single-account setup never needs --account.
    /// </summary>
    public static string? EffectiveDefault(this AccountFile file)
    {
        if (file.DefaultAccount is not null && file.Find(file.DefaultAccount) is not null)
            return file.DefaultAccount;
        return file.Accounts.Count == 1 ? file.Accounts[0].Name : null;
    }

    /// <summary>Resolves the account to use, or null when no name was given and no default applies.</summary>
    public static StoredAccount? Resolve(string? name)
    {
        var file = Load();
        var wanted = name ?? Environment.GetEnvironmentVariable("LOOPIA_ACCOUNT") ?? file.EffectiveDefault();
        if (string.IsNullOrWhiteSpace(wanted))
            return null;

        return file.Find(wanted)
            ?? throw new InvalidOperationException(
                $"No account named '{wanted}'. Run 'loopia account list' to see the configured accounts.");
    }
}
