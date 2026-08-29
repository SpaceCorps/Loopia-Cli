namespace Loopia.Console.Infrastructure;

/// <summary>
/// LoopiaAPI reports outcomes as a status string ("OK" on success, an error code otherwise),
/// sometimes wrapped in a single-element array.
/// </summary>
public static class LoopiaStatus
{
    public const string Ok = "OK";

    public static string Read(object? result) => result switch
    {
        string s => s,
        IReadOnlyList<object?> { Count: 1 } list => Read(list[0]),
        null => "UNKNOWN_ERROR",
        _ => result.ToString() ?? "UNKNOWN_ERROR"
    };

    public static void EnsureOk(object? result, string operation)
    {
        var status = Read(result);
        if (!string.Equals(status, Ok, StringComparison.Ordinal))
            throw new LoopiaApiException($"{operation} failed: {status}");
    }
}
