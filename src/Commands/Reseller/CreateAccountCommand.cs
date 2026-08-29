using System.ComponentModel;
using Loopia.Console.Infrastructure;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Loopia.Console.Commands.Reseller;

public sealed class CreateAccountCommand : AsyncCommand<CreateAccountCommand.Settings>
{
    public sealed class Settings : ApiSettings
    {
        [CommandOption("--domain <DOMAIN>")]
        [Description("Domain name used as the account username (ACE-coded for IDN domains)")]
        public required string Domain { get; init; }

        [CommandOption("--account-type <TYPE>")]
        [Description("LOOPIADOMAIN, LOOPIADNS, EMAIL_PRIVATE, STARTER, HOSTING_PRIVATE, HOSTING_BUSINESS or HOSTING_BUSINESS_PLUS")]
        public required string AccountType { get; init; }

        [CommandOption("--domain-configuration <CONFIG>")]
        [Description("NO_CONFIG, PARKING, HOSTING_UNIX, HOSTING_AUTOBAHN or HOSTING_WINDOWS (default: NO_CONFIG)")]
        public string DomainConfiguration { get; init; } = "NO_CONFIG";

        [CommandOption("--buy-domain")]
        [Description("Register the domain as part of creating the account")]
        public bool BuyDomain { get; init; }

        [CommandOption("--billing-contact-reseller")]
        [Description("The reseller handles billing instead of the customer being invoiced directly")]
        public bool BillingContactReseller { get; init; }

        [CommandOption("--tech-contact-reseller")]
        [Description("The reseller is the technical contact instead of the customer")]
        public bool TechContactReseller { get; init; }

        [CommandOption("--accept-terms")]
        [Description("Confirm that the end user has accepted the relevant agreements")]
        public bool AcceptTerms { get; init; }

        [CommandOption("--firstname <NAME>")]
        [Description("Owner contact first name")]
        public required string Firstname { get; init; }

        [CommandOption("--lastname <NAME>")]
        [Description("Owner contact last name")]
        public required string Lastname { get; init; }

        [CommandOption("--company <COMPANY>")]
        [Description("Owner contact company, empty for private individuals")]
        public string Company { get; init; } = string.Empty;

        [CommandOption("--street <STREET>")]
        [Description("Owner contact street address")]
        public required string Street { get; init; }

        [CommandOption("--street2 <STREET>")]
        [Description("Owner contact street address, second line")]
        public string Street2 { get; init; } = string.Empty;

        [CommandOption("--zip <ZIP>")]
        [Description("Owner contact postal code")]
        public required string Zip { get; init; }

        [CommandOption("--city <CITY>")]
        [Description("Owner contact city")]
        public required string City { get; init; }

        [CommandOption("--country <ISO2>")]
        [Description("Owner contact country as a two-letter ISO code, e.g. SE")]
        public required string Country { get; init; }

        [CommandOption("--orgno <ORGNO>")]
        [Description("Owner contact organisation or personal identity number")]
        public string Orgno { get; init; } = string.Empty;

        [CommandOption("--norid-pid <PID>")]
        [Description("NORID personal identifier, required only for .no domains held by private individuals")]
        public string NoridPid { get; init; } = string.Empty;

        [CommandOption("--phone <PHONE>")]
        [Description("Owner contact phone number")]
        public string Phone { get; init; } = string.Empty;

        [CommandOption("--cell <CELL>")]
        [Description("Owner contact mobile number")]
        public string Cell { get; init; } = string.Empty;

        [CommandOption("--fax <FAX>")]
        [Description("Owner contact fax number")]
        public string Fax { get; init; } = string.Empty;

        [CommandOption("--email <EMAIL>")]
        [Description("Owner contact email address")]
        public required string Email { get; init; }
    }

    public override ValidationResult Validate(CommandContext context, Settings settings) =>
        settings.AcceptTerms
            ? ValidationResult.Success()
            : ValidationResult.Error("Loopia requires the end user to have accepted the agreements; pass --accept-terms.");

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var client = settings.CreateClient();
        var contact = new Dictionary<string, object?>
        {
            ["firstname"] = settings.Firstname,
            ["lastname"] = settings.Lastname,
            ["company"] = settings.Company,
            ["street"] = settings.Street,
            ["street2"] = settings.Street2,
            ["zip"] = settings.Zip,
            ["city"] = settings.City,
            ["country_iso2"] = settings.Country,
            ["orgno"] = settings.Orgno,
            ["norid_pid"] = settings.NoridPid,
            ["phone"] = settings.Phone,
            ["cell"] = settings.Cell,
            ["fax"] = settings.Fax,
            ["email"] = settings.Email
        };

        var result = await client.CallGlobalAsync("createNewAccount",
            settings.Domain,
            contact,
            settings.BillingContactReseller,
            settings.TechContactReseller,
            settings.BuyDomain,
            settings.DomainConfiguration,
            settings.AccountType,
            true);

        YamlOutput.Write(result);
        return 0;
    }
}
