using Loopia.Console.Commands.Billing;
using Loopia.Console.Commands.Domains;
using Loopia.Console.Commands.Records;
using Loopia.Console.Commands.Reseller;
using Loopia.Console.Commands.Subdomains;
using Spectre.Console.Cli;

var app = new CommandApp();

app.Configure(config =>
{
    config.SetApplicationName("loopia");

    config.AddBranch("domains", domains =>
    {
        domains.SetDescription("Manage domain names");
        domains.AddCommand<ListDomainsCommand>("list")
            .WithDescription("List all domain names in the account");
        domains.AddCommand<GetDomainCommand>("get")
            .WithDescription("Get billing and registration details for a domain");
        domains.AddCommand<CheckDomainCommand>("check")
            .WithDescription("Check whether a domain name is available for registration");
        domains.AddCommand<OrderDomainCommand>("order")
            .WithDescription("Register a new domain name");
        domains.AddCommand<AddDomainCommand>("add")
            .WithDescription("Add an already registered domain to the account");
        domains.AddCommand<TransferDomainCommand>("transfer")
            .WithDescription("Transfer a domain to Loopia using an auth code");
        domains.AddCommand<RemoveDomainCommand>("remove")
            .WithDescription("Remove or deactivate a domain");
        domains.AddCommand<UpdateDnsServersCommand>("nameservers")
            .WithDescription("Set the name servers for a domain");
    });

    config.AddBranch("subdomains", subdomains =>
    {
        subdomains.SetDescription("Manage subdomains");
        subdomains.AddCommand<ListSubdomainsCommand>("list")
            .WithDescription("List the subdomains of a domain");
        subdomains.AddCommand<AddSubdomainCommand>("add")
            .WithDescription("Connect a subdomain to a domain");
        subdomains.AddCommand<RemoveSubdomainCommand>("remove")
            .WithDescription("Remove a subdomain");
    });

    config.AddBranch("records", records =>
    {
        records.SetDescription("Manage DNS zone records");
        records.AddCommand<ListRecordsCommand>("list")
            .WithDescription("List the zone records of a subdomain");
        records.AddCommand<AddRecordCommand>("add")
            .WithDescription("Add a zone record");
        records.AddCommand<UpdateRecordCommand>("update")
            .WithDescription("Update a zone record");
        records.AddCommand<RemoveRecordCommand>("remove")
            .WithDescription("Remove a zone record");
    });

    config.AddBranch("billing", billing =>
    {
        billing.SetDescription("Inspect credits and invoices");
        billing.AddCommand<CreditsCommand>("credits")
            .WithDescription("Get the LoopiaPrePAID balance");
        billing.AddCommand<UnpaidInvoicesCommand>("unpaid-invoices")
            .WithDescription("List unpaid invoices");
        billing.AddCommand<GetInvoiceCommand>("invoice")
            .WithDescription("Get a single invoice by reference number");
        billing.AddCommand<PayInvoiceCommand>("pay-invoice")
            .WithDescription("Pay an invoice using LoopiaPrePAID credits");
    });

    config.AddBranch("reseller", reseller =>
    {
        reseller.SetDescription("Reseller-only operations");
        reseller.AddCommand<ListCustomersCommand>("customers")
            .WithDescription("List the customers connected to the reseller account");
        reseller.AddCommand<CreateAccountCommand>("create-account")
            .WithDescription("Create a new Loopia account, optionally registering its domain");
        reseller.AddCommand<OrderStatusCommand>("order-status")
            .WithDescription("Get the status of an account creation order");
        reseller.AddCommand<TransferCreditsCommand>("transfer-credits")
            .WithDescription("Transfer credits between two customer accounts");
    });
});

return app.Run(args);
