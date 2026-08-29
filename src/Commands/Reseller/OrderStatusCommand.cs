using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Reseller;

public sealed class OrderStatusCommand : AsyncCommand<OrderStatusCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--order-reference <REFERENCE>")]
        [Description("The order reference returned by 'reseller create-account'")]
        public required string OrderReference { get; init; }
    }

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var result = await client.CallGlobalAsync("getOrderStatus", settings.OrderReference);
        YamlOutput.Write(result);
        return 0;
    }
}
