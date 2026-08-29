using System.Globalization;
using System.Xml.Linq;

namespace Loopia.Console.Infrastructure;

/// <summary>
/// Minimal XML-RPC serializer/deserializer covering the value types LoopiaAPI uses.
/// </summary>
public static class XmlRpc
{
    public static XDocument BuildRequest(string methodName, IReadOnlyList<object?> parameters)
    {
        var call = new XElement("methodCall", new XElement("methodName", methodName));
        var paramsElement = new XElement("params");
        foreach (var parameter in parameters)
            paramsElement.Add(new XElement("param", SerializeValue(parameter)));
        call.Add(paramsElement);
        return new XDocument(new XDeclaration("1.0", "UTF-8", null), call);
    }

    public static object? ParseResponse(string xml)
    {
        var root = XDocument.Parse(xml).Root
            ?? throw new LoopiaApiException("Empty response from LoopiaAPI.");

        var fault = root.Element("fault");
        if (fault is not null)
        {
            var value = ParseValue(fault.Element("value"));
            if (value is IDictionary<string, object?> members)
            {
                var code = members.TryGetValue("faultCode", out var c) ? c : null;
                var message = members.TryGetValue("faultString", out var s) ? s : null;
                throw new LoopiaApiException($"XML-RPC fault {code}: {message}");
            }
            throw new LoopiaApiException($"XML-RPC fault: {value}");
        }

        var param = root.Element("params")?.Element("param");
        return param is null ? null : ParseValue(param.Element("value"));
    }

    private static XElement SerializeValue(object? value)
    {
        var element = new XElement("value");
        switch (value)
        {
            case null:
                element.Add(new XElement("string", string.Empty));
                break;
            case string s:
                element.Add(new XElement("string", s));
                break;
            case bool b:
                element.Add(new XElement("boolean", b ? "1" : "0"));
                break;
            case int i:
                element.Add(new XElement("int", i.ToString(CultureInfo.InvariantCulture)));
                break;
            case long l:
                element.Add(new XElement("int", l.ToString(CultureInfo.InvariantCulture)));
                break;
            case double d:
                element.Add(new XElement("double", d.ToString("R", CultureInfo.InvariantCulture)));
                break;
            case IDictionary<string, object?> map:
                var structElement = new XElement("struct");
                foreach (var (name, member) in map)
                    structElement.Add(new XElement("member", new XElement("name", name), SerializeValue(member)));
                element.Add(structElement);
                break;
            case System.Collections.IEnumerable list:
                var data = new XElement("data");
                foreach (var item in list)
                    data.Add(SerializeValue(item));
                element.Add(new XElement("array", data));
                break;
            default:
                element.Add(new XElement("string", Convert.ToString(value, CultureInfo.InvariantCulture) ?? string.Empty));
                break;
        }
        return element;
    }

    private static object? ParseValue(XElement? value)
    {
        if (value is null) return null;

        var typed = value.Elements().FirstOrDefault();
        if (typed is null) return value.Value;

        return typed.Name.LocalName switch
        {
            "string" => typed.Value,
            "int" or "i4" or "i8" => long.TryParse(typed.Value, NumberStyles.Integer, CultureInfo.InvariantCulture, out var l) ? l : typed.Value,
            "boolean" => typed.Value.Trim() == "1",
            "double" => double.TryParse(typed.Value, NumberStyles.Float, CultureInfo.InvariantCulture, out var d) ? d : typed.Value,
            "dateTime.iso8601" => typed.Value,
            "base64" => typed.Value,
            "nil" => null,
            "array" => typed.Element("data")?.Elements("value").Select(ParseValue).ToList() ?? new List<object?>(),
            "struct" => typed.Elements("member").ToDictionary(
                m => m.Element("name")?.Value ?? string.Empty,
                m => ParseValue(m.Element("value"))),
            _ => typed.Value
        };
    }
}
