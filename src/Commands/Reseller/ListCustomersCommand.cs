using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Reseller;

public sealed class ListCustomersCommand : AsyncCommand<ApiSettings>
{
    public override async Task<int> ExecuteAsync(CommandContext context, ApiSettings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallGlobalAsync("getCustomers");
        YamlOutput.Write(result);
        return 0;
    }
}
