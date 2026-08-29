using System.Text;
using System.Xml.Linq;

namespace Loopia.Console.Infrastructure;

public sealed class LoopiaClient
{
    public const string DefaultEndpoint = "https://api.loopia.se/RPCSERV";

    private readonly HttpClient _http = new();
    private readonly Uri _endpoint;
    private readonly string _username;
    private readonly string _password;
    private readonly string? _customerNumber;

    public LoopiaClient(string username, string password, string? customerNumber = null, string? endpoint = null)
    {
        _username = username;
        _password = password;
        _customerNumber = string.IsNullOrWhiteSpace(customerNumber) ? null : customerNumber;
        _endpoint = new Uri(endpoint is { Length: > 0 } ? endpoint : DefaultEndpoint);
    }

    /// <summary>
    /// Calls a method that accepts the optional reseller customer number after the credentials.
    /// </summary>
    public Task<object?> CallAsync(string method, params object?[] args)
    {
        var parameters = new List<object?> { _username, _password };
        if (_customerNumber is not null)
            parameters.Add(_customerNumber);
        parameters.AddRange(args);
        return InvokeAsync(method, parameters);
    }

    /// <summary>
    /// Calls a method that only takes the credentials, with no customer number parameter.
    /// </summary>
    public Task<object?> CallGlobalAsync(string method, params object?[] args)
    {
        var parameters = new List<object?> { _username, _password };
        parameters.AddRange(args);
        return InvokeAsync(method, parameters);
    }

    private async Task<object?> InvokeAsync(string method, IReadOnlyList<object?> parameters)
    {
        var request = XmlRpc.BuildRequest(method, parameters);
        var xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>" + request.Root!.ToString(SaveOptions.DisableFormatting);
        var content = new StringContent(xml, Encoding.UTF8, "text/xml");
        var response = await _http.PostAsync(_endpoint, content);
        var body = await response.Content.ReadAsStringAsync();

        if (!response.IsSuccessStatusCode)
            throw new LoopiaApiException($"HTTP {(int)response.StatusCode}: {body}");

        return XmlRpc.ParseResponse(body);
    }
}
