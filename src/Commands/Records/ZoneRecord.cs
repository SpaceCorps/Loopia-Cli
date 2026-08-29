namespace Loopia.Console.Commands.Records;

/// <summary>
/// Builds the record_obj struct that LoopiaAPI expects for zone record calls.
/// </summary>
public static class ZoneRecord
{
    public static Dictionary<string, object?> Build(string type, string rdata, int ttl, int priority, int recordId) => new()
    {
        ["type"] = type,
        ["ttl"] = ttl,
        ["priority"] = priority,
        ["rdata"] = rdata,
        ["record_id"] = recordId
    };
}
