using System.Text.Json.Serialization;

namespace Loopia.Console.Infrastructure.Accounts;

public sealed class StoredAccount
{
    [JsonPropertyName("name")]
    public required string Name { get; set; }

    [JsonPropertyName("username")]
    public required string Username { get; set; }

    /// <summary>The password encrypted by <see cref="SecretProtector"/>, never plain text.</summary>
    [JsonPropertyName("password")]
    public required string ProtectedPassword { get; set; }

    [JsonPropertyName("customerNumber")]
    public string? CustomerNumber { get; set; }

    [JsonPropertyName("endpoint")]
    public string? Endpoint { get; set; }
}

public sealed class AccountFile
{
    [JsonPropertyName("defaultAccount")]
    public string? DefaultAccount { get; set; }

    [JsonPropertyName("accounts")]
    public List<StoredAccount> Accounts { get; set; } = [];
}
